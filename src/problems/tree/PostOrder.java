import java.util.List;
import java.util.ArrayList;
import java.util.Deque;
import java.util.ArrayDeque;
import java.util.stream.Collectors;

public class PostOrder {
    public static void main(String[] args) {
        TreeNode root = TreeNode.getTestCase();
        List<Integer> result = postorder(root);
        String str = result.stream().map(String::valueOf).collect(Collectors.joining("->", "[", "]"));
        System.out.println(str);
    }

    public static List<Integer> postorder(TreeNode root) {
        if (root == null) {
            return null;
        }
        List<Integer> result = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode current = root;
        TreeNode lastVisited = null;

        while (current != null || !stack.isEmpty()) {
            if (current != null) {
                stack.push(current);
                current = current.left;
            } else {
                TreeNode top = stack.peek();
                if (top.right != null && lastVisited != top.right) {
                    current = top.right;
                } else {
                    result.add(top.val);
                    lastVisited = stack.pop();
                    current = null; 
                }
            }
        }

        return result;
    }
}

class TreeNode {
    public int val;
    public TreeNode left;
    public TreeNode right;

    public TreeNode(){}

    public TreeNode(int val) {
        this.val = val;
    }

    public TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }

    /**
     *          1
     *      2       3
     *   4    5    6  7
     *       8 9
     */
    public static TreeNode getTestCase() {
        TreeNode tn5 = new TreeNode(5, new TreeNode(8), new TreeNode(9));
        TreeNode tn2 = new TreeNode(2, new TreeNode(4), tn5);
        TreeNode tn3 = new TreeNode(3, new TreeNode(6), new TreeNode(7));
        TreeNode tn1 = new TreeNode(1, tn2, tn3);
        return tn1;
    }
}