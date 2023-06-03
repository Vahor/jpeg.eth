// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {ERC721} from "openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import {ERC721Enumerable} from "openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";
import {POOR_MSG, NOT_OPEN_MSG, MAX_DAILY_MSG, MAX_DAILY_USER_MSG, SECONDS_PER_DAY, SAME_ACTIVE_STATE_MSG} from "./constants.sol";


contract JPEGNFT is ERC721Enumerable, Ownable {
    uint256 public constant price = 0.000001 ether;
    uint8 public constant maxPerDayAndUser = 1;
    uint8 public constant maxPerDay = 100;

    string public baseURI;

    mapping(uint256 => uint8) public mintedOnDay;
    mapping(address => mapping(uint256 => uint8)) public mintedOnDayUser;

    bool public isOpen = false;

    constructor() ERC721("Magnificient Image", "JPEG") {}

    function _baseURI() override view internal returns (string memory) {
        return baseURI;
    }

    function withdraw() public onlyOwner {
        uint balance = address(this).balance;
        payable(msg.sender).transfer(balance);
    }

    function setBaseURI(string memory value) public onlyOwner {
        baseURI = value;
    }

    function startOfDayTimestamp() private view returns (uint256) {
        unchecked {
            return block.timestamp - (block.timestamp % SECONDS_PER_DAY);
        }
    }

    function mintedTodayGlobal() public view returns (uint8)  {
        return mintedOnDay[startOfDayTimestamp()];
    }

    function _mintedTodayGlobal(uint256 day) private view returns (uint8)  {
        return mintedOnDay[day];
    }

    function mintedTodayUser(address who) public view returns (uint8)  {
        return mintedOnDayUser[who][startOfDayTimestamp()];
    }

    function _mintedTodayUser(address who, uint256 day) private view returns (uint8)  {
        return mintedOnDayUser[who][day];
    }

    function DEBUG_STEAL() public onlyOwner {
        uint256 day = startOfDayTimestamp();

        _safeMint(msg.sender, totalSupply());

        mintedOnDay[day]++;
        mintedOnDayUser[msg.sender][day]++;
    }

    function DEBUG_RESET_DAILY_USER(address who) public onlyOwner {
        uint256 day = startOfDayTimestamp();

        mintedOnDayUser[who][day] = 0;
    }

    function DEBUG_RESET_DAILY() public onlyOwner {
        uint256 day = startOfDayTimestamp();

        mintedOnDay[day] = 0;
    }

    function purchase() external payable {
        require(isOpen, NOT_OPEN_MSG);
        require(msg.value >= price, POOR_MSG);

        uint256 day = startOfDayTimestamp();

        require(_mintedTodayGlobal(day) < maxPerDay, MAX_DAILY_MSG);
        // We are checking _mintedTodayUser in beforeTokenTransfer

        _safeMint(msg.sender, totalSupply());

        mintedOnDay[day]++;
        mintedOnDayUser[msg.sender][day]++;
    }

    function setOpen(bool state) public onlyOwner {
        require(state != isOpen, SAME_ACTIVE_STATE_MSG);
        isOpen = state;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 firstTokenId,
        uint256 batchSize
    ) internal virtual override {
        super._beforeTokenTransfer(from, to, firstTokenId, batchSize);

        uint256 day = startOfDayTimestamp();

        // In any case it's illegal to have more than 1 transfer per day per user
        require(_mintedTodayUser(msg.sender, day) < maxPerDayAndUser, MAX_DAILY_USER_MSG);
    }


}
