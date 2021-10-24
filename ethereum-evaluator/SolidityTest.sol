pragma solidity ^0.8.7;
contract SolidityTest {
   constructor() {
   }

   event log (uint i);
   function getResult() public payable {
      uint a = 1;
      uint b = 2;
      uint result = a + b;
      emit log(result);
      //return result;
   }
}
