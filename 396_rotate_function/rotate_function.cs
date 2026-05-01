public class Solution {
    public int MaxRotateFunction(int[] nums) {
        int max = int.MinValue;
        for (int i = 0; i < nums.Length; i++) {
            int tmp = F(nums, i);
            if (tmp > max) {
                max = tmp;
            }
        }
        return max;
    }

    int F(int[] nums, int n) {
        int sum = 0;
        int multiplier = 0;
        for (int i = n; i < nums.Length; i++) {
            sum += nums[i] * multiplier;
            multiplier++;
        }
        for (int i = 0; i < n; i++) {
            sum += nums[i] * multiplier;
            multiplier++;
        }
        return sum;
    }
}