# Test Coverage Documentation

## 📊 Current Status

| Metric | Value |
|--------|-------|
| **Current Coverage** | ~30% |
| **Target Coverage** | 60-70% |
| **Tests Passing** | 80/80 (100%) |
| **Main Issue** | Tests only verify functions return, not actual logic |

## 🎯 Coverage Improvement Plan

We have created a comprehensive plan to increase test coverage from 30% to 60-70%.

### Documents in this Directory

1. **[IMPROVEMENT_PLAN.md](IMPROVEMENT_PLAN.md)** - Detailed implementation plan
   - 4 phases with specific tasks
   - Expected coverage gains per phase
   - Code examples and templates
   - Timeline: 3-5 days

2. **[QUICK_CHECKLIST.md](QUICK_CHECKLIST.md)** - Quick reference checklist
   - Daily goals and milestones
   - Quick start commands
   - Progress tracking
   - Test templates

## 📈 Improvement Strategy

### Phase 1: Quick Wins (1-2 days) → ~50% coverage
**Focus**: Backend, scan cache, cleanup history tests
- Add real file operation tests
- Test error handling
- Test cache invalidation logic
- **Gain**: +15-20% coverage

### Phase 2: Checker Coverage (2-3 days) → ~65% coverage
**Focus**: All 16 checker modules with real file mocks
- Test actual file detection logic
- Mock realistic cache structures
- Test size calculations
- **Gain**: +15-20% coverage

### Phase 3: Edge Cases (1-2 days) → ~70%+ coverage
**Focus**: File system edge cases and error handling
- Symlinks, permissions, unicode filenames
- Corrupted data recovery
- Concurrent operations
- **Gain**: +10-15% coverage

### Phase 4: Integration (1 day) → ~75%+ coverage
**Focus**: Full workflow tests
- End-to-end scenarios
- Multi-step operations
- **Gain**: +5-10% coverage

## 🚀 Quick Start

### Option 1: Follow the Full Plan
```bash
# Read the detailed plan
cat docs/coverage/IMPROVEMENT_PLAN.md

# Start implementing Phase 1
git checkout -b improve-test-coverage
```

### Option 2: Use the Checklist
```bash
# Follow the quick checklist
cat docs/coverage/QUICK_CHECKLIST.md

# Start checking off tasks
```

### Option 3: Start Immediately (Recommended)
```bash
# Create feature branch
git checkout -b improve-test-coverage

# Implement these 5 high-impact tests first:
# 1. test_execute_cleanup_with_temp_files (backend.rs)
# 2. test_restore_from_quarantine (cleanup_history.rs)
# 3. test_cache_ttl_expiration (scan_cache.rs)
# 4. test_invalidate_specific_items (backend.rs)
# 5. test_nodejs_cache_detection_real_files (nodejs checker)

# Check progress
cargo llvm-cov --html --open
```

## 📊 Checking Coverage

### Generate HTML Report
```bash
cargo llvm-cov --html --open
```

### Get Text Summary
```bash
cargo llvm-cov --summary-only
```

### Check Specific File
```bash
cargo llvm-cov --text | grep backend.rs
```

### View on Codecov
- Visit: https://codecov.io/gh/canggihpw/devsweep
- View file-by-file coverage
- Track trends over time

## 🎯 Target Metrics

### Recommended Coverage Targets by Module

| Module | Current | Target | Priority |
|--------|---------|--------|----------|
| `backend.rs` | ~30% | 70% | 🔴 HIGH |
| `checkers/*.rs` | ~20% | 60% | 🔴 HIGH |
| `cleanup_history.rs` | ~40% | 70% | 🔴 HIGH |
| `scan_cache.rs` | ~35% | 65% | 🟡 MEDIUM |
| `types.rs` | ~90% | 90% | ✅ GOOD |
| `utils.rs` | ~90% | 90% | ✅ GOOD |

## 💡 Key Principles

### DO ✅
- Use `TempDir` for file operations
- Test one thing per test
- Test both success and error paths
- Use realistic mock data
- Clean up resources automatically

### DON'T ❌
- Test UI code (excluded in codecov.yml)
- Compare unsigned integers with `>= 0`
- Use hardcoded paths like `/Users/...`
- Depend on system state
- Create flaky time-dependent tests

## 🏆 Success Criteria

- [ ] Coverage reaches 60-70%
- [ ] All tests passing (no failures)
- [ ] No flaky tests (100% reproducible)
- [ ] Codecov badge shows correct %
- [ ] CI workflows remain green
- [ ] Coverage maintains on new code

## 📝 Progress Tracking

### Milestones
- [ ] **40%** - Initial improvements
- [ ] **50%** - Phase 1 complete (Quick Wins)
- [ ] **60%** - Phase 2 halfway (Checkers started)
- [ ] **65%** - Phase 2 complete (All checkers)
- [ ] **70%** - Phase 3 complete (Edge cases) 🎉

### Recommended First Week
- **Day 1**: Read plan, setup branch, add 6 backend tests → 45%
- **Day 2**: Add 6 cleanup history tests → 50%
- **Day 3**: Add 5 checker tests (Node.js, Docker) → 55%
- **Day 4**: Add 5 more checker tests (Python, Rust, Xcode) → 62%
- **Day 5**: Add 3 edge case tests → 65%

## 🔗 Related Documentation

- [Main Testing Guide](../TESTING.md) - Overall testing strategy
- [Project Summary](../PROJECT_SUMMARY.md) - Project overview
- Coverage CI: `.github/workflows/coverage.yml`
- Codecov Config: `codecov.yml`

## 📞 Questions?

### "Where should I start?"
→ Read [IMPROVEMENT_PLAN.md](IMPROVEMENT_PLAN.md) Phase 1 and implement the first 3 tests

### "How do I know what's not covered?"
→ Run `cargo llvm-cov --html --open` and look for red lines in the HTML report

### "Should we hit 100% coverage?"
→ No! 60-70% is excellent. Focus on critical paths, not every edge case.

### "What if tests are flaky?"
→ Use deterministic test data with `TempDir`. Avoid timing-dependent assertions.

### "Can I skip UI testing?"
→ Yes! UI is excluded in `codecov.yml`. Focus on business logic.

---

**Ready to improve coverage?** Start with [IMPROVEMENT_PLAN.md](IMPROVEMENT_PLAN.md)! 🚀

**Current**: 30% coverage, 80 tests  
**Target**: 65% coverage, ~120 tests  
**Timeline**: 3-5 days