#[doc = "Register `MIPI_ADD_DATA_FIFO` reader"]
pub type R = crate::R<MipiAddDataFifoSpec>;
#[doc = "Field `ADD_DATA_FIFO` reader - lowest 4 bytes in additional data fifo; reading\n\nincrements fifo read pointer.\n\nFirst embedded data byte will be written to bits 7:0 of\n\n32-bit data word, second data byte written to 15:8 etc.\n\n"]
pub type AddDataFifoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - lowest 4 bytes in additional data fifo; reading\n\nincrements fifo read pointer.\n\nFirst embedded data byte will be written to bits 7:0 of\n\n32-bit data word, second data byte written to 15:8 etc.\n\n"]
    #[inline(always)]
    pub fn add_data_fifo(&self) -> AddDataFifoR {
        AddDataFifoR::new(self.bits)
    }
}
#[doc = "Additional Data Fifo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_fifo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiAddDataFifoSpec;
impl crate::RegisterSpec for MipiAddDataFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_add_data_fifo::R`](R) reader structure"]
impl crate::Readable for MipiAddDataFifoSpec {}
#[doc = "`reset()` method sets MIPI_ADD_DATA_FIFO to value 0"]
impl crate::Resettable for MipiAddDataFifoSpec {
    const RESET_VALUE: u32 = 0;
}
