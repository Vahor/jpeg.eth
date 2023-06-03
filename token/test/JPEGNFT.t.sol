// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, stdStorage, StdStorage} from "forge-std/Test.sol";
import {ERC721} from "openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import {Strings} from "openzeppelin-contracts/contracts/utils/Strings.sol";

import {JPEGNFT} from "../src/JPEGNFT.sol";
import {POOR_MSG, NOT_OPEN_MSG, MAX_DAILY_MSG, MAX_DAILY_USER_MSG, SECONDS_PER_DAY, ONLY_OWNER_MSG, SAME_ACTIVE_STATE_MSG} from "../src/constants.sol";

contract JPEFNFTTtest is Test {
    using Strings for uint256;

    using stdStorage for StdStorage;

    JPEGNFT private token;
    uint private price1;

    address private owner = makeAddr("owner");
    address private user = makeAddr("user");

    function setUp() public {
        vm.startPrank(owner);
        token = new JPEGNFT();
        price1 = token.price();
        vm.stopPrank();
    }

    function open() public {
        vm.startPrank(owner);
        token.setOpen(true);
        assertEq(token.isOpen(), true);
        vm.stopPrank();
    }

    function testIsOpen() public {
        assertEq(token.isOpen(), false);

        vm.expectRevert(abi.encodePacked(NOT_OPEN_MSG));
        token.purchase();

        open();

        vm.startPrank(user);
        vm.expectRevert(abi.encodePacked(POOR_MSG));
        token.purchase{value: 0}();

        vm.deal(user, token.price());
        token.purchase{value: token.price()}();

        assertEq(token.balanceOf(user), 1);
        assertEq(address(token).balance, token.price());
        assertEq(token.mintedTodayGlobal(), 1);
        assertEq(token.mintedTodayUser(user), 1);
        vm.stopPrank();
    }

    function testBaseURI() public {
        string memory baseURI = "https://test.com/";

        // Empty by default
        assertEq(token.baseURI(), "");

        // Only owner
        vm.expectRevert(abi.encodePacked(ONLY_OWNER_MSG));
        token.setBaseURI(baseURI);
        assertEq(token.baseURI(), "", "baseURI should not be set");

        vm.startPrank(owner);
        token.setBaseURI(baseURI);
        assertEq(token.baseURI(), baseURI, "baseURI should be set");
        vm.stopPrank();

        // test tokenURI
        open();

        startHoax(user, price1);
        token.purchase{value: price1}();

        uint256 ownedToken = token.tokenOfOwnerByIndex(user, 0);
        string memory a = token.tokenURI(ownedToken);
        assertEq(a, string(abi.encodePacked(baseURI, "0")), "tokenURI should be set");

    }

    function testWithdraw() public {
        // add money on the contract
        uint money = 100;
        startHoax(address(token), money);

        // Only owner
        vm.expectRevert(abi.encodePacked(ONLY_OWNER_MSG));
        token.withdraw();
        assertEq(address(token).balance, money);

        // Success
        vm.startPrank(owner);
        token.withdraw();
        assertEq(address(token).balance, 0);
        assertEq(owner.balance, money);

    }

    function testSetOpen() public {

        vm.expectRevert(abi.encodePacked(ONLY_OWNER_MSG));
        token.setOpen(true);
        assertEq(token.isOpen(), false);

        vm.startPrank(owner);
        token.setOpen(true);
        assertEq(token.isOpen(), true);

        vm.expectRevert(abi.encodePacked(SAME_ACTIVE_STATE_MSG));
        token.setOpen(true);
        assertEq(token.isOpen(), true);


        token.setOpen(false);
        assertEq(token.isOpen(), false);

        vm.stopPrank();
    }

    function testPurchase() public {
        open();

        startHoax(user, price1);
        token.purchase{value: price1}();

        assertEq(token.balanceOf(user), 1, "Incorrect balance after purchase");
        assertEq(token.mintedTodayGlobal(), 1, "Incorrect minted today global count");
        assertEq(token.mintedTodayUser(user), 1, "Incorrect minted today user count");

        // another user
        address user2 = makeAddr("user2");
        startHoax(user2, price1);
        token.purchase{value: price1}();

        assertEq(token.balanceOf(user2), 1, "Incorrect balance after purchase");
        assertEq(token.mintedTodayGlobal(), 2, "Incorrect minted today global count");
        assertEq(token.mintedTodayUser(user2), 1, "Incorrect minted today user count");
    }

    function testMintedOnDayUserLimit() public {
        // Try to buy 2 tokens in the same day, the second one should fail
        // Warp one day, then try to buy another one, it should work

        open();

        vm.startPrank(user);
        vm.deal(user, price1 * 2);

        token.purchase{value: price1}();

        vm.expectRevert(abi.encodePacked(MAX_DAILY_USER_MSG));
        token.purchase{value: price1}();

        vm.warp(block.timestamp + SECONDS_PER_DAY);
        token.purchase{value: price1}();

        vm.stopPrank();

        assertEq(token.balanceOf(user), 2, "Incorrect balance after purchase");
        assertEq(token.mintedTodayGlobal(), 1, "Incorrect minted today global count"); // On the second day, the global count should be reset
        assertEq(token.mintedTodayUser(user), 1, "Incorrect minted today user count");
    }

    function testMintedOnDayGlobalLimit() public {
        // Try to buy max-1 token, all of them should work
        // Try to buy max token, it should fail

        open();

        for (uint256 i = 0; i < token.maxPerDay(); i++) {
            address u = makeAddr(i.toString());
            vm.startPrank(u);
            vm.deal(u, token.price());

            token.purchase{value: price1}();

            assertEq(token.balanceOf(u), 1, "Incorrect balance after purchase");
            assertEq(token.mintedTodayGlobal(), i + 1, "Incorrect minted today global count");
            assertEq(token.mintedTodayUser(u), 1, "Incorrect minted today user count");

            vm.stopPrank();
        }

        assertEq(token.mintedTodayGlobal(), token.maxPerDay(), "Incorrect minted today global count");

        vm.startPrank(user);
        vm.deal(user, token.price());

        vm.expectRevert(abi.encodePacked(MAX_DAILY_MSG));
        token.purchase{value: price1}();

        vm.stopPrank();
    }

    function testBeforeTokenTransfer() public {
        // Two users, user1 buys a token, transfer it to user2, user2 tries to buy a token => it should fail

        open();

        address user2 = makeAddr("user2");

        vm.startPrank(user);
        vm.deal(user, token.price());

        token.purchase{value: price1}();

        // transfer token to user2
        token.transferFrom(user, user2, 0);
        assertEq(token.balanceOf(user), 0, "Incorrect balance after transfer");
        assertEq(token.balanceOf(user2), 1, "Incorrect balance after transfer");

        vm.stopPrank();

        // user2 tries to buy a token
        vm.startPrank(user2);
        vm.deal(user2, token.price());

        vm.expectRevert(abi.encodePacked(MAX_DAILY_USER_MSG));
        token.purchase{value: price1}();
    }



}
