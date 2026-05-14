public class Solution {
    public bool IsGood(int[] nums) {
        BitArray bits = new BitArray(nums.Length);
        foreach (int num in nums)
        {
            if (num > nums.Length-1) return false;
            
            if (bits[num-1]) 
            {
                if (num == nums.Length-1 && !bits[num])
                {
                    bits[num] = true;
                }
                else
                {
                    return false;
                }
            }
            bits[num-1] = true;
        }
        foreach (bool bit in bits)
        {
            if (!bit) return false;
        }
        return true;
    }
}