#[doc = "Register `MI_STATUS` reader"]
pub type R = crate::R<MiStatusSpec>;
#[doc = "Field `mp_y_fifo_full` reader - FIFO full flag of Y FIFO in main path asserted since last\n\nclear\n\n"]
pub type MpYFifoFullR = crate::BitReader;
#[doc = "Field `mp_cb_fifo_full` reader - FIFO full flag of Cb FIFO in main path asserted since\n\nlast clear"]
pub type MpCbFifoFullR = crate::BitReader;
#[doc = "Field `mp_cr_fifo_full` reader - FIFO full flag of Cr FIFO in main path asserted since\n\nlast clear"]
pub type MpCrFifoFullR = crate::BitReader;
#[doc = "Field `sp_y_fifo_full` reader - FIFO full flag of Y FIFO in self path asserted since last\n\nclear"]
pub type SpYFifoFullR = crate::BitReader;
#[doc = "Field `sp_cb_fifo_full` reader - FIFO full flag of Cb FIFO in self path asserted since last\n\nclear"]
pub type SpCbFifoFullR = crate::BitReader;
#[doc = "Field `sp_cr_fifo_full` reader - FIFO full flag of Cr FIFO in self path asserted since last\n\nclear"]
pub type SpCrFifoFullR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FIFO full flag of Y FIFO in main path asserted since last\n\nclear\n\n"]
    #[inline(always)]
    pub fn mp_y_fifo_full(&self) -> MpYFifoFullR {
        MpYFifoFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO full flag of Cb FIFO in main path asserted since\n\nlast clear"]
    #[inline(always)]
    pub fn mp_cb_fifo_full(&self) -> MpCbFifoFullR {
        MpCbFifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO full flag of Cr FIFO in main path asserted since\n\nlast clear"]
    #[inline(always)]
    pub fn mp_cr_fifo_full(&self) -> MpCrFifoFullR {
        MpCrFifoFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO full flag of Y FIFO in self path asserted since last\n\nclear"]
    #[inline(always)]
    pub fn sp_y_fifo_full(&self) -> SpYFifoFullR {
        SpYFifoFullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO full flag of Cb FIFO in self path asserted since last\n\nclear"]
    #[inline(always)]
    pub fn sp_cb_fifo_full(&self) -> SpCbFifoFullR {
        SpCbFifoFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO full flag of Cr FIFO in self path asserted since last\n\nclear"]
    #[inline(always)]
    pub fn sp_cr_fifo_full(&self) -> SpCrFifoFullR {
        SpCrFifoFullR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "MI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiStatusSpec;
impl crate::RegisterSpec for MiStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_status::R`](R) reader structure"]
impl crate::Readable for MiStatusSpec {}
#[doc = "`reset()` method sets MI_STATUS to value 0"]
impl crate::Resettable for MiStatusSpec {
    const RESET_VALUE: u32 = 0;
}
