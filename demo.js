var majorityElement = function(nums) {
  if (nums.length === 1) return nums[0];

  let map = new Map();

  for (let i = 0; i < nums.length; i++) {
    let current_value = map.get(nums[i]);
    if (current_value === undefined) {
        map.set(nums[i], 1);
    } else {
        map.set(nums[i], current_value+1)
        if (current_value + 1 > nums.length / 2) {
            return nums[i];
        }
    }
  }
};


const a = majorityElement([2,2,1,1,1,2,2]);
console.log(a);
