#[doc = "Register `DENALI_CTL_01` reader"]
pub type R = crate::R<DenaliCtl01Spec>;
#[doc = "Field `MAX_ROW_REG` reader - Holds the maximum width of memory address bus."]
pub type MaxRowRegR = crate::FieldReader;
#[doc = "Field `MAX_COL_REG` reader - Holds the maximum width of column address in DRAMs."]
pub type MaxColRegR = crate::FieldReader;
#[doc = "Field `MAX_CS_REG` reader - Holds the maximum number of chip selects available."]
pub type MaxCsRegR = crate::FieldReader;
#[doc = "Field `READ_DATA_FIFO_DEPTH` reader - Reports the depth of the controller core read data queue."]
pub type ReadDataFifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Holds the maximum width of memory address bus."]
    #[inline(always)]
    pub fn max_row_reg(&self) -> MaxRowRegR {
        MaxRowRegR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Holds the maximum width of column address in DRAMs."]
    #[inline(always)]
    pub fn max_col_reg(&self) -> MaxColRegR {
        MaxColRegR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Holds the maximum number of chip selects available."]
    #[inline(always)]
    pub fn max_cs_reg(&self) -> MaxCsRegR {
        MaxCsRegR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Reports the depth of the controller core read data queue."]
    #[inline(always)]
    pub fn read_data_fifo_depth(&self) -> ReadDataFifoDepthR {
        ReadDataFifoDepthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_01::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl01Spec;
impl crate::RegisterSpec for DenaliCtl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_01::R`](R) reader structure"]
impl crate::Readable for DenaliCtl01Spec {}
#[doc = "`reset()` method sets DENALI_CTL_01 to value 0x0002_0c10"]
impl crate::Resettable for DenaliCtl01Spec {
    const RESET_VALUE: u32 = 0x0002_0c10;
}
