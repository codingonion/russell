#include <inttypes.h>
#include <stdlib.h>

#include "klu.h"

#include "constants.h"

/// @brief Holds the data for KLU
struct InterfaceKLU {
    /// @brief Holds control parameters and statistics
    klu_common common;

    /// @brief Holds the pre-ordering computed by klu_analyze
    klu_symbolic *symbolic;

    /// @brief Holds the factors computed by klu_factor
    klu_numeric *numeric;

    /// @brief indicates that the initialization has been completed
    C_BOOL initialization_completed;

    /// @brief Indicates that the factorization (at least once) has been completed
    C_BOOL factorization_completed;
};

/// @brief Allocates a new KLU interface
struct InterfaceKLU *solver_klu_new() {
    struct InterfaceKLU *solver = (struct InterfaceKLU *)malloc(sizeof(struct InterfaceKLU));

    if (solver == NULL) {
        return NULL;
    }

    solver->symbolic = NULL;
    solver->numeric = NULL;
    solver->initialization_completed = C_FALSE;
    solver->factorization_completed = C_FALSE;

    return solver;
}

/// @brief Deallocates the KLU interface
void solver_klu_drop(struct InterfaceKLU *solver) {
    if (solver == NULL) {
        return;
    }

    if (solver->symbolic != NULL) {
        klu_free_symbolic(&solver->symbolic, &solver->common);
        free(solver->symbolic);
    }
    if (solver->numeric != NULL) {
        klu_free_numeric(&solver->numeric, &solver->common);
        free(solver->numeric);
    }

    free(solver);
}

/// @brief Performs the symbolic factorization
int32_t solver_klu_initialize(struct InterfaceKLU *solver,
                              int32_t ordering,
                              int32_t scaling,
                              int32_t ndim,
                              const int32_t *col_pointers,
                              const int32_t *row_indices) {
    if (solver == NULL) {
        return ERROR_NULL_POINTER;
    }

    if (solver->initialization_completed == C_TRUE) {
        return ERROR_ALREADY_INITIALIZED;
    }

    klu_defaults(&solver->common);

    if (ordering >= 0) {
        solver->common.ordering = ordering;
    }

    if (scaling >= 0) {
        solver->common.scale = scaling;
    }

    // remove "const" here assuming that klu will not change those variables
    solver->symbolic = klu_analyze(ndim,
                                   (int32_t *)col_pointers,
                                   (int32_t *)row_indices,
                                   &solver->common);
    if (solver->symbolic == NULL) {
        return KLU_ERROR_ANALYZE;
    }

    solver->initialization_completed = C_TRUE;

    return SUCCESSFUL_EXIT;
}

/// @brief Performs the numeric factorization
int32_t solver_klu_factorize(struct InterfaceKLU *solver,
                             int32_t *effective_ordering,
                             int32_t *effective_scaling,
                             double *cond_estimate,
                             C_BOOL compute_cond,
                             const int32_t *col_pointers,
                             const int32_t *row_indices,
                             const double *values) {
    if (solver == NULL) {
        return ERROR_NULL_POINTER;
    }

    if (solver->initialization_completed == C_FALSE) {
        return ERROR_NEED_INITIALIZATION;
    }

    if (solver->factorization_completed == C_TRUE) {
        // free the previous numeric to avoid memory leak
        klu_free_numeric(&solver->numeric, &solver->common);
        solver->numeric = NULL;
    }

    // remove "const" here assuming that klu will not change those variables
    solver->numeric = klu_factor((int32_t *)col_pointers,
                                 (int32_t *)row_indices,
                                 (double *)values,
                                 solver->symbolic,
                                 &solver->common);
    if (solver->numeric == NULL) {
        return KLU_ERROR_FACTOR;
    }

    // save ordering and scaling
    *effective_ordering = solver->common.ordering;
    *effective_scaling = solver->common.scale;

    // reciprocal condition number estimate
    if (compute_cond == C_TRUE) {
        int status = klu_condest((int32_t *)col_pointers,
                                 (double *)values,
                                 solver->symbolic,
                                 solver->numeric,
                                 &solver->common);
        if (status == C_FALSE) {
            return KLU_ERROR_COND_EST;
        }
        *cond_estimate = solver->common.condest;
    }

    solver->factorization_completed = C_TRUE;

    return SUCCESSFUL_EXIT;
}

/// @brief Computes the solution of the linear system
int32_t solver_klu_solve(struct InterfaceKLU *solver,
                         int32_t ndim,
                         double *in_rhs_out_x) {
    if (solver == NULL) {
        return ERROR_NULL_POINTER;
    }

    if (solver->factorization_completed == C_FALSE) {
        return ERROR_NEED_FACTORIZATION;
    }

    klu_solve(solver->symbolic,
              solver->numeric,
              ndim,
              1,
              in_rhs_out_x,
              &solver->common);

    return SUCCESSFUL_EXIT;
}
