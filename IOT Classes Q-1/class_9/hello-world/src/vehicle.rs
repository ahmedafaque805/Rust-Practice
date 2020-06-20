mod vehicles() {

#[derive(Debug)]
struct vehicle{
    name: String,
    company_name: String,
    color: String,
    transmission: String,
    model: String,
}
impl vehicle{
    fn vehicle_spec(self) ->String {
        let vehicle = {self.name,self.company_name,self.color,self.transmission,self.model};
        vehicle
    }
}
}