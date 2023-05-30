// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, stdStorage, StdStorage} from "forge-std/Test.sol";
import {ERC721} from "openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import {Strings} from "openzeppelin-contracts/contracts/utils/Strings.sol";

import {MythNFT} from "../src/MythNFT.sol";
import {POOR_MSG, NOT_OPEN_MSG, MAX_DAILY_MSG, MAX_DAILY_USER_MSG, DAY_MS, ONLY_OWNER_MSG} from "../src/constants.sol";

contract MythNFTTest is Test {
    using Strings for uint256;

    using stdStorage for StdStorage;

    MythNFT private token;

    address private owner = makeAddr("owner");
    address private user = makeAddr("user");

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

        vm.deal(user, token.price());
        token.purchase{value: token.price()}();
        vm.stopPrank();
    }

    function open() public {
        vm.startPrank(owner);
        token.setActive(true);
        assertEq(token.saleIsActive(), true);
        vm.stopPrank();
    }

    function test_limit_daily() public {
        assertEq(token.mintedTodayGlobal(), 0);
        assertEq(token.mintedTodayUser(user), 0);

        open();
        uint price1 = token.price();

        uint max = token.maxPerDay();
        startHoax(user, price1 * max);
        assertEq(user.balance, price1 * max);

        token.purchase{value: price1}();
        assertEq(user.balance, price1 * max - price1);
        assertEq(token.mintedTodayUser(user), 1);
        assertEq(token.mintedTodayGlobal(), 1);
        assertEq(token.balanceOf(user), 1);

        vm.expectRevert(abi.encodePacked(MAX_DAILY_USER_MSG));
        token.purchase{value: price1}();

        vm.warp(block.timestamp + DAY_MS);
        assertEq(token.mintedTodayUser(user), 0);
        assertEq(token.mintedTodayGlobal(), 0);

        for (uint256 i = 0; i < token.maxPerDay(); i++) {
            address u = makeAddr(i.toString());
            vm.startPrank(u);
            vm.deal(u, token.price());
            token.purchase{value: price1}();
            vm.stopPrank();
        }

        assertEq(token.mintedTodayGlobal(), token.maxPerDay());


        vm.startPrank(user);
        vm.expectRevert(abi.encodePacked(MAX_DAILY_MSG));
        token.purchase{value: price1}();
        vm.stopPrank();

    }

    function test_uri() public  {
        open();

        string memory baseURI = "https://test.com/";
        vm.startPrank(owner);
        token.setBaseURI(baseURI);
        assertEq(token.baseURI(), baseURI);
        vm.stopPrank();

        uint price1 = token.price();
        startHoax(user, price1);
        token.purchase{value: price1}();

        uint256 ownedTokens = token.balanceOf(user);
        for(uint256 i = 0; i < ownedTokens; i++) {
            uint256 ownedToken = token.tokenOfOwnerByIndex(user, i);
            string memory a = token.tokenURI(ownedToken);
            assertEq(a, string(abi.encodePacked(baseURI, i.toString())));
        }

    }

    function test_withdraw() public {
        uint money = 100;
        startHoax(address(token), money);
        vm.expectRevert(abi.encodePacked(ONLY_OWNER_MSG));
        token.withdraw();
        assertEq(address(token).balance, money);

        vm.startPrank(owner);
        token.withdraw();
        assertEq(address(token).balance, 0);

    }


}
