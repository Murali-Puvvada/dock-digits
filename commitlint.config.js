module.exports = {
    extends: ['@commitlint/config-conventional'],
    rules: {
      'header-pattern': [
        2,
        'always',
        /^(feat|fix|chore|docs|refactor|test|style): MUR-\d+ .{1,50}$/
      ],
      'header-max-length': [2, 'always', 72],
    },
};