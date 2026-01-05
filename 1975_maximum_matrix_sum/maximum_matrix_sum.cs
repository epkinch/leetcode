public class Solution {
    public long MaxMatrixSum(int[][] matrix) {
        int n = matrix.Length;

        int min = int.MaxValue;
        int negative_count = 0;
        long absolute_sum = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {

                if (Math.Abs(matrix[i][j]) < min) {
                    min = Math.Abs(matrix[i][j]);
                }

                if (matrix[i][j] < 0) {
                    negative_count++;
                }

                absolute_sum += Math.Abs(matrix[i][j]);
            }
        }

        if (negative_count % 2 == 0) {
            return absolute_sum;
        }
        else {
            return absolute_sum - 2*min;
        }
    }
}