#[doc = "Register `DENALI_CTL_02` reader"]
pub type R = crate::R<DenaliCtl02Spec>;
#[doc = "Field `READ_DATA_FIFO_PTR_WIDTH` reader - Reports the width of the controller core read data queue pointer."]
pub type ReadDataFifoPtrWidthR = crate::FieldReader;
#[doc = "Field `WRITE_DATA_FIFO_DEPTH` reader - Reports the depth of the controller core write data latency queue."]
pub type WriteDataFifoDepthR = crate::FieldReader;
#[doc = "Field `WRITE_DATA_FIFO_PTR_WIDTH` reader - Reports the width of the controller core write data latency queue pointer."]
pub type WriteDataFifoPtrWidthR = crate::FieldReader;
#[doc = "Field `MEMCD_RMODW_FIFO_DEPTH` reader - Reports the depth of the controller core read/modify/write FIFO."]
pub type MemcdRmodwFifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reports the width of the controller core read data queue pointer."]
    #[inline(always)]
    pub fn read_data_fifo_ptr_width(&self) -> ReadDataFifoPtrWidthR {
        ReadDataFifoPtrWidthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reports the depth of the controller core write data latency queue."]
    #[inline(always)]
    pub fn write_data_fifo_depth(&self) -> WriteDataFifoDepthR {
        WriteDataFifoDepthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reports the width of the controller core write data latency queue pointer."]
    #[inline(always)]
    pub fn write_data_fifo_ptr_width(&self) -> WriteDataFifoPtrWidthR {
        WriteDataFifoPtrWidthR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reports the depth of the controller core read/modify/write FIFO."]
    #[inline(always)]
    pub fn memcd_rmodw_fifo_depth(&self) -> MemcdRmodwFifoDepthR {
        MemcdRmodwFifoDepthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_02::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl02Spec;
impl crate::RegisterSpec for DenaliCtl02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_02::R`](R) reader structure"]
impl crate::Readable for DenaliCtl02Spec {}
#[doc = "`reset()` method sets DENALI_CTL_02 to value 0"]
impl crate::Resettable for DenaliCtl02Spec {
    const RESET_VALUE: u32 = 0;
}
