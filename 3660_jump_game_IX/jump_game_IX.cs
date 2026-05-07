public class Solution {
    public int[] MaxValue(int[] nums) {
        int[] ans = new int[nums.Length];

        var indexed = nums
            .Select((value, index) => new { value, index })
            .OrderBy(x => x.value)
            .ToArray();

        foreach (var jumper in indexed) 
        {
            ans[jumper.index] = jumper.value;
            foreach (var jumped in indexed)
            {
                if ((jumper.index < jumped.index && 
                    jumper.value > jumper.index) ||
                    (jumper.index > jumped.index &&
                    jumper.value < jumped.value))
                {
                    ans[jumper.index] = jumped.value;
                    break;
                }
                else if (jumper.value )

            }
        }
        return ans;
    }
}