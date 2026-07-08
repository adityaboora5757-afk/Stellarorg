/**
 * Pure client-side utility functions for Tranche Escrow operations.
 */

/**
 * Truncates a Stellar address to a readable format (e.g. GBGX...I7U).
 */
export function truncateAddress(address: string): string {
  if (!address) return "";
  if (address.length <= 10) return address;
  return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
}

interface MilestoneInput {
  amount: number;
  is_completed: boolean;
}

/**
 * Calculates the remaining locked capital in the escrow vault.
 */
export function calculateRemainingAllocation(total: number, milestones: MilestoneInput[]): number {
  const completedAmount = milestones
    .filter(m => m.is_completed)
    .reduce((sum, m) => sum + m.amount, 0);
  const remaining = total - completedAmount;
  return remaining < 0 ? 0 : remaining;
}

/**
 * Formats a raw stroop amount into a formatted XLM string.
 * 1 XLM = 10,000,000 stroops.
 */
export function formatXlmAmount(stroops: bigint | number): string {
  const amount = Number(stroops) / 10000000;
  return `${amount.toFixed(2)} XLM`;
}
