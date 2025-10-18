Main types → sub-types → when to use

Opposite-Ends (Converging)

Pair Sum on Sorted Array (move l/r by comparing sum to target)

Palindrome Check (compare s[l] vs s[r], skip non-alnum if needed)

Reverse In-Place (swap l/r, move inward)

Three-Sum (inner step) (fix i, then two-pointer on i+1..)

Same-Direction (Fast/Slow; “Tortoise–Hare”)

Cycle Detection in Linked List (Floyd’s algorithm)

Find Middle of List (slow + fast = 2× speed)

Kth from End (lead pointer K ahead, then move both)

Window Bounds (Left/Right)

Longest/Shortest Subarray meeting a condition (grow r, shrink l)

At-Most-K constraint (e.g., ≤K zeros, ≤K distinct chars)

Streaming filters (maintain counts/sets while moving the window)

Partitioning (Read/Write or Hoare/Lomuto-style)

Remove/Keep In-Place (read pointer scans; write pointer compacts)

Deduplicate Sorted (skip repeats; write unique)

Predicate Partition (e.g., evens left/odds right; 0/1 segregation)

(3-way Dutch National Flag is a 3-pointer variant of this family)

Bidirectional Merge / Set Ops

Merge Two Sorted Arrays (advance the smaller)

Intersection/Union/Difference of sorted lists

Squaring & Sorting (largest magnitude from ends, fill from back)

Pattern “components” (how to design one cleanly)

Data model: array/slice, string, or linked list

Pointer init: (l=0, r=n-1), (slow=head, fast=head), or (l=r=0)

Invariant: statement that stays true (e.g., window satisfies constraint)

Move rules: exactly when to move which pointer (if/else based)

Stop rule: l > r, fast == null, window can’t improve, etc.

Outcome: index(es), boolean, count/length, or in-place transformed data

Preconditions: sortedness, monotonicity, hashable keys for windows

Complexity: usually O(n) time, O(1) space (except maps for windows)

Edge cases: empty, single element, all equal, negatives, Unicode, overflow