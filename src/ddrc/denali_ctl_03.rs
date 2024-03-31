#[doc = "Register `DENALI_CTL_03` reader"]
pub type R = crate::R<DenaliCtl03Spec>;
#[doc = "Field `MEMCD_RMODW_FIFO_PTR_WIDTH` reader - Reports the width of the controller core read/modify/write FIFO pointer."]
pub type MemcdRmodwFifoPtrWidthR = crate::FieldReader;
#[doc = "Field `DENALI0_CMDFIFO_LOG2_DEPTH` reader - Reports the depth of the DENALI port 0 command FIFO. Value is the log2 value of the depth."]
pub type Denali0CmdfifoLog2DepthR = crate::FieldReader;
#[doc = "Field `DENALI0_RMODWFIFO_LOG2_DEPTH` reader - Reports the depth of the DENALI port 0 read/modify/write FIFO. Value is the log2 value of the depth."]
pub type Denali0RmodwfifoLog2DepthR = crate::FieldReader;
#[doc = "Field `DENALI0_WRFIFO_LOG2_DEPTH` reader - Reports the depth of the DENALI port 0 write data FIFO. Value is the log2 value of the depth."]
pub type Denali0WrfifoLog2DepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reports the width of the controller core read/modify/write FIFO pointer."]
    #[inline(always)]
    pub fn memcd_rmodw_fifo_ptr_width(&self) -> MemcdRmodwFifoPtrWidthR {
        MemcdRmodwFifoPtrWidthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reports the depth of the DENALI port 0 command FIFO. Value is the log2 value of the depth."]
    #[inline(always)]
    pub fn denali0_cmdfifo_log2_depth(&self) -> Denali0CmdfifoLog2DepthR {
        Denali0CmdfifoLog2DepthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reports the depth of the DENALI port 0 read/modify/write FIFO. Value is the log2 value of the depth."]
    #[inline(always)]
    pub fn denali0_rmodwfifo_log2_depth(&self) -> Denali0RmodwfifoLog2DepthR {
        Denali0RmodwfifoLog2DepthR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reports the depth of the DENALI port 0 write data FIFO. Value is the log2 value of the depth."]
    #[inline(always)]
    pub fn denali0_wrfifo_log2_depth(&self) -> Denali0WrfifoLog2DepthR {
        Denali0WrfifoLog2DepthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_03::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl03Spec;
impl crate::RegisterSpec for DenaliCtl03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_03::R`](R) reader structure"]
impl crate::Readable for DenaliCtl03Spec {}
#[doc = "`reset()` method sets DENALI_CTL_03 to value 0"]
impl crate::Resettable for DenaliCtl03Spec {
    const RESET_VALUE: u32 = 0;
}
