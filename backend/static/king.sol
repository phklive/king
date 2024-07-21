// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

contract King {
    address public king;
    uint96 public lastBlock;

    uint256 internal constant PRICE = 1 ether;
    uint256 internal constant NEXT_DELAY = 10;

    error TooLow();
    error AlreadyWon();
    error DidntWin();
    error TransferFailed();

    constructor() payable {
        lastBlock = uint96(block.number);
    }

    function payIn() external payable {
        if (won()) revert AlreadyWon();
        if (msg.value != PRICE) revert TooLow();
        king = msg.sender;
        lastBlock = uint96(max(block.number + NEXT_DELAY, lastBlock));
    }

    function payOut() external payable {
        if (king != msg.sender || !won()) revert DidntWin();
        (bool suc,) = msg.sender.call{value: address(this).balance}("");
        if (!suc) revert TransferFailed();
    }

    function won() public view returns (bool) {
        return king != address(0) && block.number >= lastBlock;
    }

    function max(uint256 x, uint256 y) internal pure returns (uint256 z) {
        assembly {
            z := xor(mul(gt(x, y), xor(x, y)), y)
        }
    }
}
