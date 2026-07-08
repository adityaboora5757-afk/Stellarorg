import { describe, it, expect } from "vitest";
import { 
  truncateAddress, 
  calculateRemainingAllocation, 
  formatXlmAmount 
} from "./escrowUtils";

describe("Escrow Utility Functions", () => {
  
  // Test 1: Address truncation formatting
  it("should truncate Stellar addresses correctly", () => {
    const address = "GBGXPRIFPNXXZG2A36TSSK5TKPGPP3PQQ4DSKW7EO4JDNPMEEV7SDI7U";
    expect(truncateAddress(address)).toBe("GBGXPR...DI7U");
    expect(truncateAddress("")).toBe("");
    expect(truncateAddress("ABCD")).toBe("ABCD");
  });

  // Test 2: Locked allocation computation
  it("should calculate remaining locked escrow pool allocation", () => {
    const total = 300;
    const milestones = [
      { amount: 100, is_completed: true },
      { amount: 100, is_completed: false },
      { amount: 100, is_completed: false },
    ];
    
    // 1 completed milestone of 100 XLM, remaining should be 200 XLM
    expect(calculateRemainingAllocation(total, milestones)).toBe(200);

    // All milestones completed, remaining should be 0
    const completedMilestones = milestones.map(m => ({ ...m, is_completed: true }));
    expect(calculateRemainingAllocation(total, completedMilestones)).toBe(0);
  });

  // Test 3: Stroops unit formatting
  it("should convert raw stroops to formatted XLM strings", () => {
    expect(formatXlmAmount(100000000)).toBe("10.00 XLM");
    expect(formatXlmAmount(5000000)).toBe("0.50 XLM");
    expect(formatXlmAmount(0)).toBe("0.00 XLM");
  });

});
