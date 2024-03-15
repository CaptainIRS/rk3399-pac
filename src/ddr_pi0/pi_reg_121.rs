#[doc = "Register `PI_REG_121` reader"]
pub type R = crate::R<PiReg121Spec>;
#[doc = "Register `PI_REG_121` writer"]
pub type W = crate::W<PiReg121Spec>;
#[doc = "Field `PI_WDQLVL_PERIODIC` reader - Enables write DQ training periodic."]
pub type PiWdqlvlPeriodicR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_PERIODIC` writer - Enables write DQ training periodic."]
pub type PiWdqlvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_REQ` writer - Indicates software write DQ training request."]
pub type PiWdqlvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_CS` reader - Indicates write DQ training target chip select."]
pub type PiWdqlvlCsR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_CS` writer - Indicates write DQ training target chip select."]
pub type PiWdqlvlCsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TDFI_WDQLVL_EN` reader - Indicates DFI timing param - enable to FIFO write."]
pub type PiTdfiWdqlvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_WDQLVL_EN` writer - Indicates DFI timing param - enable to FIFO write."]
pub type PiTdfiWdqlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enables write DQ training periodic."]
    #[inline(always)]
    pub fn pi_wdqlvl_periodic(&self) -> PiWdqlvlPeriodicR {
        PiWdqlvlPeriodicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - Indicates write DQ training target chip select."]
    #[inline(always)]
    pub fn pi_wdqlvl_cs(&self) -> PiWdqlvlCsR {
        PiWdqlvlCsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Indicates DFI timing param - enable to FIFO write."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_en(&self) -> PiTdfiWdqlvlEnR {
        PiTdfiWdqlvlEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables write DQ training periodic."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_periodic(&mut self) -> PiWdqlvlPeriodicW<PiReg121Spec> {
        PiWdqlvlPeriodicW::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates software write DQ training request."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_req(&mut self) -> PiWdqlvlReqW<PiReg121Spec> {
        PiWdqlvlReqW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Indicates write DQ training target chip select."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_cs(&mut self) -> PiWdqlvlCsW<PiReg121Spec> {
        PiWdqlvlCsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates DFI timing param - enable to FIFO write."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_en(&mut self) -> PiTdfiWdqlvlEnW<PiReg121Spec> {
        PiTdfiWdqlvlEnW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 121\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_121::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_121::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg121Spec;
impl crate::RegisterSpec for PiReg121Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_121::R`](R) reader structure"]
impl crate::Readable for PiReg121Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_121::W`](W) writer structure"]
impl crate::Writable for PiReg121Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_121 to value 0"]
impl crate::Resettable for PiReg121Spec {
    const RESET_VALUE: u32 = 0;
}
