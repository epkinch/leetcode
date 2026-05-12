public class Solution {
    public int MinimumEffort(int[][] tasks) {
        var sorted_tasks = tasks
        .OrderByDescending(x => x[1] - x[0])
        .ToArray();

        int ans = sorted_tasks[0][0];
        int tmp = ans;
        foreach (int[] task in sorted_tasks) {
            int after_task = tmp - task[1];
            if (after_task < 0) {
                
                tmp -= after_task;
                ans -= after_task;
            }
            tmp = tmp - task[0];
        }
        return ans;
    }
}