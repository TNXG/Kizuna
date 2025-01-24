import antfu from "@antfu/eslint-config";

export default antfu({
	formatters: true,
	vue: true,
	stylistic: {
		indent: "tab",
		quotes: "double",
		semi: true,
	},
	rules: {
		"antfu/top-level-function": "off",
	},
});
