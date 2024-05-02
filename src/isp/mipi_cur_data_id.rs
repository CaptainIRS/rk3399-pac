#[doc = "Register `MIPI_CUR_DATA_ID` reader"]
pub type R = crate::R<MipiCurDataIdSpec>;
#[doc = "Field `DATA_TYPE` reader - data type of currently received packet"]
pub type DataTypeR = crate::FieldReader;
#[doc = "Field `VIRTUAL_CHANNEL` reader - virtual channel of currently received packet"]
pub type VirtualChannelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - data type of currently received packet"]
    #[inline(always)]
    pub fn data_type(&self) -> DataTypeR {
        DataTypeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - virtual channel of currently received packet"]
    #[inline(always)]
    pub fn virtual_channel(&self) -> VirtualChannelR {
        VirtualChannelR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Current Data Identifier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_cur_data_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiCurDataIdSpec;
impl crate::RegisterSpec for MipiCurDataIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_cur_data_id::R`](R) reader structure"]
impl crate::Readable for MipiCurDataIdSpec {}
#[doc = "`reset()` method sets MIPI_CUR_DATA_ID to value 0"]
impl crate::Resettable for MipiCurDataIdSpec {
    const RESET_VALUE: u32 = 0;
}
