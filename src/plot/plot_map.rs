use plotters::prelude::*;
use std::f64::consts::PI as Pi;

const OUT_FILE_NAME: &str = "./map.gif";
pub fn plot_map(rays:Vec<Vec<(f64,f64,f64)>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 100)?.into_drawing_area();

    let n_pitch = 100;
    for pitch in 0..n_pitch {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Map", ("sans-serif", 20))
            .build_cartesian_3d(-10.0..10.0, -10.0..10.0, -10.0..10.0)?;
        chart.with_projection(|mut p| {
            p.yaw = 2.0*Pi*f64::from(pitch)/f64::from(n_pitch);
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        let data = vec![vec![(1.0,2.0,2.0),(3.0,0.0,1.0)],vec![(3.0,5.0,2.0),(3.0,0.0,1.0)]];
    
        for ray in rays.clone(){
            chart.draw_series(LineSeries::new(
                ray,
                &RED,
            ))?;
        }
        root.present()?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}