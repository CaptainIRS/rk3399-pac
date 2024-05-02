#[doc = "Register `MIPI_GEN_SHORT_A_B` reader"]
pub type R = crate::R<MipiGenShortABSpec>;
#[doc = "Field `data_field_A` reader - 16 bit user defined data field from last generic short\n\npacket of data type 0xA"]
pub type DataFieldAR = crate::FieldReader<u16>;
#[doc = "Field `data_field_B` reader - 16 bit user defined data field from last generic\n\nshort packet of data type 0xB"]
pub type DataFieldBR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16 bit user defined data field from last generic short\n\npacket of data type 0xA"]
    #[inline(always)]
    pub fn data_field_a(&self) -> DataFieldAR {
        DataFieldAR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 16 bit user defined data field from last generic\n\nshort packet of data type 0xB"]
    #[inline(always)]
    pub fn data_field_b(&self) -> DataFieldBR {
        DataFieldBR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "data field for generic short packets of data type 0xA and 0xB\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_a_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiGenShortABSpec;
impl crate::RegisterSpec for MipiGenShortABSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_gen_short_a_b::R`](R) reader structure"]
impl crate::Readable for MipiGenShortABSpec {}
#[doc = "`reset()` method sets MIPI_GEN_SHORT_A_B to value 0"]
impl crate::Resettable for MipiGenShortABSpec {
    const RESET_VALUE: u32 = 0;
}
