#[doc = "Register `MIPI_GEN_SHORT_E_F` reader"]
pub type R = crate::R<MipiGenShortEFSpec>;
#[doc = "Field `data_field_E` reader - 16 bit user defined data field from last generic short\n\npacket of data type 0xE"]
pub type DataFieldER = crate::FieldReader<u16>;
#[doc = "Field `data_field_F` reader - 16 bit user defined data field from last generic short\n\npacket of data type 0xF"]
pub type DataFieldFR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16 bit user defined data field from last generic short\n\npacket of data type 0xE"]
    #[inline(always)]
    pub fn data_field_e(&self) -> DataFieldER {
        DataFieldER::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 16 bit user defined data field from last generic short\n\npacket of data type 0xF"]
    #[inline(always)]
    pub fn data_field_f(&self) -> DataFieldFR {
        DataFieldFR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "data field for generic short packets of data type 0xE and 0xF\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n\n\n \n\n \n\n\n\nThis is the control register for AF measurement unit Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_AFM_BASE + 0000H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n31:\n\n\n\n1 \n\n\n\n--- unused \n\n\n\n0 afm_en AF measurement enable \n\n\n\n0: AF measurement is \n\n\n\ndisabled 1: AF \n\n\n\nmeasurement is enabled \n\n\n\nWriting a 1 to this register starts a new measurement \n\n\n\nand resets the afm_fin (measurement finished) interrupt \n\n\n\nto 0. \n\n\n\nAs long as the afm_en is 1, the AFM unit \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_e_f::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiGenShortEFSpec;
impl crate::RegisterSpec for MipiGenShortEFSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_gen_short_e_f::R`](R) reader structure"]
impl crate::Readable for MipiGenShortEFSpec {}
#[doc = "`reset()` method sets MIPI_GEN_SHORT_E_F to value 0"]
impl crate::Resettable for MipiGenShortEFSpec {
    const RESET_VALUE: u32 = 0;
}
