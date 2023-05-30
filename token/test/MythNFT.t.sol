// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, stdStorage, StdStorage} from "forge-std/Test.sol";
import {ERC721} from "openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import {Strings} from "openzeppelin-contracts/contracts/utils/Strings.sol";

import {MythNFT} from "../src/MythNFT.sol";
import {POOR_MSG, NOT_OPEN_MSG, MAX_DAILY_MSG, MAX_DAILY_USER_MSG, DAY_MS} from "../src/constants.sol";

contract TestNFTTest is Test {
    using stdStorage for StdStorage;

    MythNFT token;

    address owner = makeAddr("owner");
    address user = makeAddr("user");


    function setUp() public {
        vm.startPrank(owner);
        token = new MythNFT();
        vm.stopPrank();
    }

    function test_isActive() public {
        assertEq(token.saleIsActive(), false);

        vm.expectRevert(abi.encodePacked(NOT_OPEN_MSG));
        token.purchase();

        open();

        vm.startPrank(user);
        vm.expectRevert(abi.encodePacked(POOR_MSG));
        token.purchase{value: 0}();

        vm.deal(user, token.PRICE());
        token.purchase{value: token.PRICE()}();
        vm.stopPrank();
    }

    function open() private {
        vm.startPrank(owner);
        token.setActive(true);
        vm.stopPrank();
    }

    function test_limit_daily() public {
        assertEq(token.mintedTodayGlobal(), 0);
        assertEq(token.mintedTodayUser(user), 0);

        open();
        uint price1 = token.PRICE();

        uint max = token.MAX_PER_DAY();
        startHoax(user, price1 * max);
        assertEq(user.balance, price1 * max);

        token.purchase{value: price1}();
        assertEq(user.balance, price1 * max - price1);
        assertEq(token.mintedTodayUser(user), 1);
        assertEq(token.mintedTodayGlobal(), 1);

        vm.expectRevert(abi.encodePacked(MAX_DAILY_USER_MSG));
        token.purchase{value: price1}();

        vm.warp(block.timestamp + DAY_MS);
        assertEq(token.mintedTodayUser(user), 0);
        assertEq(token.mintedTodayGlobal(), 0);

        for (uint256 i = 0; i < token.MAX_PER_DAY(); i++) {
            address u = makeAddr(Strings.toString(i));
            vm.startPrank(u);
            vm.deal(u, token.PRICE());
            token.purchase{value: price1}();
            vm.stopPrank();
        }

        assertEq(token.mintedTodayGlobal(), token.MAX_PER_DAY());


        vm.startPrank(user);
        vm.expectRevert(abi.encodePacked(MAX_DAILY_MSG));
        token.purchase{value: price1}();
        vm.stopPrank();

    }


}
