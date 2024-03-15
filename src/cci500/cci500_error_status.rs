#[doc = "Register `CCI500_ERROR_STATUS` reader"]
pub type R = crate::R<Cci500ErrorStatusSpec>;
#[doc = "Register `CCI500_ERROR_STATUS` writer"]
pub type W = crate::W<Cci500ErrorStatusSpec>;
#[doc = "Field `IMPRECISE_ERR_MST0` reader - Imprecise error indicator for master interface 0 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst0R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_MST0` writer - Imprecise error indicator for master interface 0 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_MST1` reader - Imprecise error indicator for master interface 1 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst1R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_MST1` writer - Imprecise error indicator for master interface 1 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_MST2` reader - Imprecise error indicator for master interface 2 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst2R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_MST2` writer - Imprecise error indicator for master interface 2 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_MST3` reader - Imprecise error indicator for master interface 3 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst3R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_MST3` writer - Imprecise error indicator for master interface 3 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_MST4` reader - Imprecise error indicator for master interface 4 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst4R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_MST4` writer - Imprecise error indicator for master interface 4 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_MST5` reader - Imprecise error indicator for master interface 5 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst5R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_MST5` writer - Imprecise error indicator for master interface 5 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
pub type ImpreciseErrMst5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_SLV0` reader - Imprecise error indicator for slave interface 0. 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv0R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_SLV0` writer - Imprecise error indicator for slave interface 0. 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_SLV1` reader - Imprecise error indicator for slave interface 1 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv1R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_SLV1` writer - Imprecise error indicator for slave interface 1 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_SLV2` reader - Imprecise error indicator for slave interface 2 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv2R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_SLV2` writer - Imprecise error indicator for slave interface 2 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_SLV3` reader - Imprecise error indicator for slave interface 3 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv3R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_SLV3` writer - Imprecise error indicator for slave interface 3 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_SLV4` reader - Imprecise error indicator for slave interface 4 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv4R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_SLV4` writer - Imprecise error indicator for slave interface 4 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_SLV5` reader - Imprecise error indicator for slave interface 5 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv5R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_SLV5` writer - Imprecise error indicator for slave interface 5 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISE_ERR_SLV6` reader - Imprecise error indicator for slave interface 6 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv6R = crate::BitReader;
#[doc = "Field `IMPRECISE_ERR_SLV6` writer - Imprecise error indicator for slave interface 6 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
pub type ImpreciseErrSlv6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Imprecise error indicator for master interface 0 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn imprecise_err_mst0(&self) -> ImpreciseErrMst0R {
        ImpreciseErrMst0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Imprecise error indicator for master interface 1 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn imprecise_err_mst1(&self) -> ImpreciseErrMst1R {
        ImpreciseErrMst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Imprecise error indicator for master interface 2 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn imprecise_err_mst2(&self) -> ImpreciseErrMst2R {
        ImpreciseErrMst2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Imprecise error indicator for master interface 3 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn imprecise_err_mst3(&self) -> ImpreciseErrMst3R {
        ImpreciseErrMst3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Imprecise error indicator for master interface 4 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn imprecise_err_mst4(&self) -> ImpreciseErrMst4R {
        ImpreciseErrMst4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Imprecise error indicator for master interface 5 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn imprecise_err_mst5(&self) -> ImpreciseErrMst5R {
        ImpreciseErrMst5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Imprecise error indicator for slave interface 0. 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn imprecise_err_slv0(&self) -> ImpreciseErrSlv0R {
        ImpreciseErrSlv0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Imprecise error indicator for slave interface 1 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn imprecise_err_slv1(&self) -> ImpreciseErrSlv1R {
        ImpreciseErrSlv1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Imprecise error indicator for slave interface 2 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn imprecise_err_slv2(&self) -> ImpreciseErrSlv2R {
        ImpreciseErrSlv2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Imprecise error indicator for slave interface 3 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn imprecise_err_slv3(&self) -> ImpreciseErrSlv3R {
        ImpreciseErrSlv3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Imprecise error indicator for slave interface 4 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn imprecise_err_slv4(&self) -> ImpreciseErrSlv4R {
        ImpreciseErrSlv4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Imprecise error indicator for slave interface 5 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn imprecise_err_slv5(&self) -> ImpreciseErrSlv5R {
        ImpreciseErrSlv5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Imprecise error indicator for slave interface 6 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn imprecise_err_slv6(&self) -> ImpreciseErrSlv6R {
        ImpreciseErrSlv6R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Imprecise error indicator for master interface 0 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst0(&mut self) -> ImpreciseErrMst0W<Cci500ErrorStatusSpec> {
        ImpreciseErrMst0W::new(self, 0)
    }
    #[doc = "Bit 1 - Imprecise error indicator for master interface 1 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst1(&mut self) -> ImpreciseErrMst1W<Cci500ErrorStatusSpec> {
        ImpreciseErrMst1W::new(self, 1)
    }
    #[doc = "Bit 2 - Imprecise error indicator for master interface 2 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst2(&mut self) -> ImpreciseErrMst2W<Cci500ErrorStatusSpec> {
        ImpreciseErrMst2W::new(self, 2)
    }
    #[doc = "Bit 3 - Imprecise error indicator for master interface 3 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst3(&mut self) -> ImpreciseErrMst3W<Cci500ErrorStatusSpec> {
        ImpreciseErrMst3W::new(self, 3)
    }
    #[doc = "Bit 4 - Imprecise error indicator for master interface 4 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst4(&mut self) -> ImpreciseErrMst4W<Cci500ErrorStatusSpec> {
        ImpreciseErrMst4W::new(self, 4)
    }
    #[doc = "Bit 5 - Imprecise error indicator for master interface 5 0: No error from the time this bit was last reset. 1: An error response has been received, but not signalled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst5(&mut self) -> ImpreciseErrMst5W<Cci500ErrorStatusSpec> {
        ImpreciseErrMst5W::new(self, 5)
    }
    #[doc = "Bit 16 - Imprecise error indicator for slave interface 0. 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv0(&mut self) -> ImpreciseErrSlv0W<Cci500ErrorStatusSpec> {
        ImpreciseErrSlv0W::new(self, 16)
    }
    #[doc = "Bit 17 - Imprecise error indicator for slave interface 1 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv1(&mut self) -> ImpreciseErrSlv1W<Cci500ErrorStatusSpec> {
        ImpreciseErrSlv1W::new(self, 17)
    }
    #[doc = "Bit 18 - Imprecise error indicator for slave interface 2 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv2(&mut self) -> ImpreciseErrSlv2W<Cci500ErrorStatusSpec> {
        ImpreciseErrSlv2W::new(self, 18)
    }
    #[doc = "Bit 19 - Imprecise error indicator for slave interface 3 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv3(&mut self) -> ImpreciseErrSlv3W<Cci500ErrorStatusSpec> {
        ImpreciseErrSlv3W::new(self, 19)
    }
    #[doc = "Bit 20 - Imprecise error indicator for slave interface 4 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv4(&mut self) -> ImpreciseErrSlv4W<Cci500ErrorStatusSpec> {
        ImpreciseErrSlv4W::new(self, 20)
    }
    #[doc = "Bit 21 - Imprecise error indicator for slave interface 5 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv5(&mut self) -> ImpreciseErrSlv5W<Cci500ErrorStatusSpec> {
        ImpreciseErrSlv5W::new(self, 21)
    }
    #[doc = "Bit 22 - Imprecise error indicator for slave interface 6 0b0: No error from the time this bit was last reset. 0b1: An error response has been received, but not signaled precisely."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv6(&mut self) -> ImpreciseErrSlv6W<Cci500ErrorStatusSpec> {
        ImpreciseErrSlv6W::new(self, 22)
    }
}
#[doc = "Imprecise Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500ErrorStatusSpec;
impl crate::RegisterSpec for Cci500ErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_error_status::R`](R) reader structure"]
impl crate::Readable for Cci500ErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cci500_error_status::W`](W) writer structure"]
impl crate::Writable for Cci500ErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_ERROR_STATUS to value 0"]
impl crate::Resettable for Cci500ErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
