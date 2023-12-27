let array = [3, 5, -4, 8, 11, 1, -1, 6];
let targetSum = 10;

function twoNumberSum(array, targetSum) {
	for (let i=0; i<array.length;i++){
		firstNum=array[i]
		for (let y=i+1; y<array.length;y++){
			secondNum = array[y]
			if (firstNum+secondNum===targetSum) return [firstNum,secondNum]
		}
	}
	return "not found"
}

let solution = twoNumberSum(array,targetSum)
console.log(solution)

function twoNumberSumHash(array, targetSum) {
	let nums = {};
	for (let num of array) {
		let possibleMatch = targetSum - num;
		if (nums.hasOwnProperty(possibleMatch)) return [num, possibleMatch];
		else nums[num] = true;
	}
	return "not found";
}

let solutionHash = twoNumberSumHash(array, targetSum)
console.log(solutionHash)

function twoNumberSumMarker(array ,targetSum){
	array.sort((a, b) => a - b);
	left=0
	right=array.length-1
	while (left < right ) {
		let sum = array[left] + array[right]
		if (sum===targetSum) return [array[left], array[right]]
		else if  (sum < targetSum) left ++
		else right -- 
	}
	return "not found"
}

let solutionMarker = twoNumberSumMarker(array,targetSum)
console.log(solutionMarker)