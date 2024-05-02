#[doc = "Register `MI_STATUS_CLR` writer"]
pub type W = crate::W<MiStatusClrSpec>;
#[doc = "Field `mp_y_fifo_full` writer - Clear status of Y FIFO full flag in main path\n\n"]
pub type MpYFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mp_cb_fifo_full` writer - Clear status of Cb FIFO full flag in main path"]
pub type MpCbFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mp_cr_fifo_full` writer - Clear status of Cr FIFO full flag in main path"]
pub type MpCrFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_y_fifo_full` writer - Clear status of Y FIFO full flag in self path\n\n"]
pub type SpYFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_cb_fifo_full` writer - Clear status of Cb FIFO full flag in self path"]
pub type SpCbFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_cr_fifo_full` writer - Clear status of Cr FIFO full flag in self path"]
pub type SpCrFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear status of Y FIFO full flag in main path\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn mp_y_fifo_full(&mut self) -> MpYFifoFullW<MiStatusClrSpec> {
        MpYFifoFullW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear status of Cb FIFO full flag in main path"]
    #[inline(always)]
    #[must_use]
    pub fn mp_cb_fifo_full(&mut self) -> MpCbFifoFullW<MiStatusClrSpec> {
        MpCbFifoFullW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear status of Cr FIFO full flag in main path"]
    #[inline(always)]
    #[must_use]
    pub fn mp_cr_fifo_full(&mut self) -> MpCrFifoFullW<MiStatusClrSpec> {
        MpCrFifoFullW::new(self, 2)
    }
    #[doc = "Bit 4 - Clear status of Y FIFO full flag in self path\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn sp_y_fifo_full(&mut self) -> SpYFifoFullW<MiStatusClrSpec> {
        SpYFifoFullW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear status of Cb FIFO full flag in self path"]
    #[inline(always)]
    #[must_use]
    pub fn sp_cb_fifo_full(&mut self) -> SpCbFifoFullW<MiStatusClrSpec> {
        SpCbFifoFullW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear status of Cr FIFO full flag in self path"]
    #[inline(always)]
    #[must_use]
    pub fn sp_cr_fifo_full(&mut self) -> SpCrFifoFullW<MiStatusClrSpec> {
        SpCrFifoFullW::new(self, 6)
    }
}
#[doc = "MI Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_status_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiStatusClrSpec;
impl crate::RegisterSpec for MiStatusClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mi_status_clr::W`](W) writer structure"]
impl crate::Writable for MiStatusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_STATUS_CLR to value 0"]
impl crate::Resettable for MiStatusClrSpec {
    const RESET_VALUE: u32 = 0;
}
