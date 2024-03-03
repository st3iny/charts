pub fn write_chart(chart: &Chart) -> Result<()> {
    let mut file = File::create("Chart.yaml")?;
    let mut buf = Vec::new();
    serde_yaml::to_writer(&mut buf, chart)?;
    file.write_all(&buf)?;
    Ok(())
}
