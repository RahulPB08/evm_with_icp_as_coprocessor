// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Coprocessor} from "../../src/Coprocessor.sol";
import {MyScript} from "../../script/Coprocessor.s.sol";

contract CoprocessorTest is Test {
    MyScript deployCoprocessor;
    address coprocessor;
    address public PLAYER = makeAddr("player");
    address public OTHER_PLAYER = makeAddr("player_2");

    function setUp() external {
        deployCoprocessor = new MyScript();
        vm.deal(PLAYER, 100000000000000000 * 10);
        vm.deal(vm.addr(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80), 100000000000000000 * 10);
        coprocessor = deployCoprocessor.run(PLAYER);
    }

    function testupdateCoprocessorCanNotCallByother() public {
        vm.prank(OTHER_PLAYER);
        vm.expectRevert("Only the coprocessor can call this function");
        Coprocessor(coprocessor).updateCoprocessor(OTHER_PLAYER);
    }
}
