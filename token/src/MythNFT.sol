// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {ERC721} from "openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import {Ownable} from"openzeppelin-contracts/contracts/access/Ownable.sol";
import {POOR_MSG, NOT_OPEN_MSG, MAX_DAILY_MSG, MAX_DAILY_USER_MSG, DAY_MS} from "./constants.sol";


contract MythNFT is ERC721, Ownable {
    uint16 public totalSupply = 0;

    uint256 public constant PRICE = 1 ether;
    uint8 public constant MAX_PER_DAY_USER = 1;
    uint8 public constant MAX_PER_DAY = 100;

    mapping(uint256 => uint8) public mintedOnDay;
    mapping(address => mapping(uint256 => uint8)) public mintedOnDayUser;

    bool public saleIsActive = false;

    constructor() ERC721("Myth", "MYTH") {}

    function withdraw() public onlyOwner {
        uint balance = address(this).balance;
        payable(msg.sender).transfer(balance);
    }

    function rounded_to_day() private view returns (uint256) {
        unchecked {
            return block.timestamp - (block.timestamp % DAY_MS);
        }
    }

    function mintedTodayGlobal() public view returns (uint8)  {
        return mintedOnDay[rounded_to_day()];
    }

    function mintedTodayUser(address who) public view returns (uint8)  {
        return mintedOnDayUser[who][rounded_to_day()];
    }

    function purchase() external payable {
        require(saleIsActive, NOT_OPEN_MSG);
        require(mintedTodayGlobal() < MAX_PER_DAY, MAX_DAILY_MSG);
        require(mintedTodayUser(msg.sender) < MAX_PER_DAY_USER, MAX_DAILY_USER_MSG);
        require(msg.value >= PRICE, POOR_MSG);

        _safeMint(msg.sender, ++totalSupply);

        uint256 day = rounded_to_day();
        mintedOnDay[day]++;
        mintedOnDayUser[msg.sender][day]++;
    }

    function setActive(bool state) public onlyOwner {
        require(state != saleIsActive, "Already in this state");
        saleIsActive = state;
    }
}
