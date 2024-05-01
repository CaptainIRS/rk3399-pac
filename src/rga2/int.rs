#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Field `SW_INTR_ERR` reader - Error interrupt flag"]
pub type SwIntrErrR = crate::BitReader;
#[doc = "Field `SW_INTR_MMU` reader - MMU interrupt"]
pub type SwIntrMmuR = crate::BitReader;
#[doc = "Field `SW_INTR_AF` reader - All command finished interrupt flag"]
pub type SwIntrAfR = crate::BitReader;
#[doc = "Field `SW_INTR_CF` reader - Current command finished interrupt flag"]
pub type SwIntrCfR = crate::BitReader;
#[doc = "Field `SW_INTR_ERR_CLR` writer - Error interrupt clear"]
pub type SwIntrErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTR_MMU_CLR` writer - MMU interrupt clear"]
pub type SwIntrMmuClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTR_AF_CLR` writer - All command finished interrupt clear"]
pub type SwIntrAfClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTR_CF_CLR` writer - Current command finished interrupt clear"]
pub type SwIntrCfClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTR_ERR_E` reader - Error interrupt enable"]
pub type SwIntrErrER = crate::BitReader;
#[doc = "Field `SW_INTR_ERR_E` writer - Error interrupt enable"]
pub type SwIntrErrEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTR_MMU_E` reader - MMU interrupt enable"]
pub type SwIntrMmuER = crate::BitReader;
#[doc = "Field `SW_INTR_MMU_E` writer - MMU interrupt enable"]
pub type SwIntrMmuEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTR_AF_E` reader - All command finished interrupt enable"]
pub type SwIntrAfER = crate::BitReader;
#[doc = "Field `SW_INTR_AF_E` writer - All command finished interrupt enable"]
pub type SwIntrAfEW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error interrupt flag"]
    #[inline(always)]
    pub fn sw_intr_err(&self) -> SwIntrErrR {
        SwIntrErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMU interrupt"]
    #[inline(always)]
    pub fn sw_intr_mmu(&self) -> SwIntrMmuR {
        SwIntrMmuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - All command finished interrupt flag"]
    #[inline(always)]
    pub fn sw_intr_af(&self) -> SwIntrAfR {
        SwIntrAfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Current command finished interrupt flag"]
    #[inline(always)]
    pub fn sw_intr_cf(&self) -> SwIntrCfR {
        SwIntrCfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn sw_intr_err_e(&self) -> SwIntrErrER {
        SwIntrErrER::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMU interrupt enable"]
    #[inline(always)]
    pub fn sw_intr_mmu_e(&self) -> SwIntrMmuER {
        SwIntrMmuER::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - All command finished interrupt enable"]
    #[inline(always)]
    pub fn sw_intr_af_e(&self) -> SwIntrAfER {
        SwIntrAfER::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_err_clr(&mut self) -> SwIntrErrClrW<IntSpec> {
        SwIntrErrClrW::new(self, 4)
    }
    #[doc = "Bit 5 - MMU interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_mmu_clr(&mut self) -> SwIntrMmuClrW<IntSpec> {
        SwIntrMmuClrW::new(self, 5)
    }
    #[doc = "Bit 6 - All command finished interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_af_clr(&mut self) -> SwIntrAfClrW<IntSpec> {
        SwIntrAfClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Current command finished interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_cf_clr(&mut self) -> SwIntrCfClrW<IntSpec> {
        SwIntrCfClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_err_e(&mut self) -> SwIntrErrEW<IntSpec> {
        SwIntrErrEW::new(self, 8)
    }
    #[doc = "Bit 9 - MMU interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_mmu_e(&mut self) -> SwIntrMmuEW<IntSpec> {
        SwIntrMmuEW::new(self, 9)
    }
    #[doc = "Bit 10 - All command finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intr_af_e(&mut self) -> SwIntrAfEW<IntSpec> {
        SwIntrAfEW::new(self, 10)
    }
}
#[doc = "RGA interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
