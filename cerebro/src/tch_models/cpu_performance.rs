use tch::{nn, Kind, Tensor};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct CpuPerformanceModelBuilder<'a> {
    group: &'a Tensor,
    clock_speed_max: &'a Tensor,
    tdp: &'a Tensor,
}

pub struct CpuPerformanceModel {
    num_groups: i64,
    clock_speed_max_map: Tensor,
    tdp_map: Tensor,
    bias_map: Tensor,
}

impl CpuPerformanceModel {
    pub fn new(p: nn::Path, num_groups: i64) -> Self {
        Self {
            num_groups,
            clock_speed_max_map: p.randn("clock_speed_max_map", &[num_groups], 0.5, 0.5),
            tdp_map: p.randn("tdp_map", &[num_groups], 0.5, 0.5),
            bias_map: p.randn("bias_map", &[num_groups], 0.5, 0.5),
        }
    }

    pub fn forward(&self, builder: &CpuPerformanceModelBuilder) -> Tensor {
        (builder.group.one_hot(self.num_groups) * &self.clock_speed_max_map).sum_dim_intlist(
            &[1],
            false,
            Kind::Double,
        ) * builder.clock_speed_max
            + (builder.group.one_hot(self.num_groups) * &self.tdp_map).sum_dim_intlist(
                &[1],
                false,
                Kind::Double,
            ) * builder.tdp
            + (builder.group.one_hot(self.num_groups) * &self.bias_map).sum_dim_intlist(
                &[1],
                false,
                Kind::Double,
            )
    }
}
