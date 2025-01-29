// Ankit
class Solution {
public:
vector<vector<int>> threeSum(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    vector<vector<int>> r;

    for (int i = 0; i < nums.size() - 2; i++) {
        if (i > 0 && nums[i] == nums[i - 1]) continue;

        int l = i + 1;
        int ri = nums.size() - 1; 

        while (l < ri) {
            int s = nums[i] + nums[l] + nums[ri];

            if (s == 0) {
                r.push_back({nums[i], nums[l], nums[ri]});
                while (l < ri && nums[l] == nums[l + 1]) l++;
                while (l < ri && nums[ri] == nums[ri - 1]) ri--;
                l++;
                ri--;
            } else if (s < 0) {
                l++;
            } else {
                ri--;
            }
        }
    }

    return r;
}
};