//#[macro_use]
//extern crate rulinalg;
//extern crate rusty_machine as rm;
//extern crate num as libnum;

pub mod learning {
    pub mod dbscan;
    pub mod lin_reg;
    pub mod k_means;
    pub mod gp;
    pub mod knn;
    pub mod pca;

    pub mod optim {
    	pub mod grad_desc;
    }
}

pub mod datasets;
