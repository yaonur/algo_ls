some_array = [3, 5, -4, 8, 11, 1, -1, 6]
targetSum = 10


# O(n*2) time | O(1) space


def twoNumberSum(array, targetSum):
    for i in range(len(array)-1):
        firstNum = array[i]
        for j in range(i+1, len(array)):
            secondNum = array[j]
            if firstNum + secondNum == targetSum:
                return [firstNum, secondNum]
    return []


solution = twoNumberSum(some_array, targetSum)
print(solution)

# O(n) time | O(n) space


def twoNumberSumHash(array, targetSum):
    nums = {}
    for num in array:
        potentialMatch = targetSum - num
        if potentialMatch in nums:
            return [potentialMatch, num]
        else:
            nums[num] = True
    return []


hash_solution = twoNumberSumHash(some_array, targetSum)
print(hash_solution)

# O(nLog(n)) time | O(1) space
def twoNumberSumMarker(array, targetSum):
    array.sort()
    left = 0
    right = len(array) - 1
    while left < right:
        currentSum = array[left] + array[right]
        if currentSum == targetSum:
            return [array[left], array[right]]
        elif currentSum < targetSum:
            left += 1
        else:
            right += 1
    return []
marker_solition = twoNumberSumMarker(some_array,targetSum)
print(marker_solition)