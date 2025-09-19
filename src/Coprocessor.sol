// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract Coprocessor {
    uint256 job_id = 0;
    address payable private coprocessor;

    constructor() {
        coprocessor = payable(msg.sender);
    }

    mapping(uint256 => uint256) public jobs;

    event NewJob(uint256 indexed job_id);

    // Function to create a new job
    function newJob() public payable returns (uint256) {
        // Require at least 0.01 ETH to be sent with the call
        require(msg.value >= 0.01 ether, "Minimum 0.01 ETH not met");

        // Forward the ETH received to the coprocessor address
        // to pay for the submission of the job result back to the EVM
        // contract.
        coprocessor.transfer(msg.value);

        // Emit the new job event
        emit NewJob(job_id);

        // Increment job counter
        job_id++;
    }

    function getResult(uint256 _job_id) public view returns(uint256) {
        return jobs[_job_id];
    }

    function callback_icp(uint256 result, uint256 job_id)
        require(msg.sender == coprocessor, "Only the coprocessor can call this function");
        jobs[job_id] = result;
    }

    function updateCoprocessor(address _coprocessor) public {
        require(
            msg.sender == coprocessor,
            "Only the coprocessor can call this function"
        );
        coprocessor = payable(_coprocessor);
    }
}
