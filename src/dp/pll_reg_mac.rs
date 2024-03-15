#[doc = "Register `PLL_REG_MAC` reader"]
pub type R = crate::R<PllRegMacSpec>;
#[doc = "Register `PLL_REG_MAC` writer"]
pub type W = crate::W<PllRegMacSpec>;
#[doc = "Field `ANALOG_BACKUP1` reader - Reserved"]
pub type AnalogBackup1R = crate::FieldReader;
#[doc = "Field `ANALOG_BACKUP1` writer - Reserved"]
pub type AnalogBackup1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn analog_backup1(&self) -> AnalogBackup1R {
        AnalogBackup1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn analog_backup1(&mut self) -> AnalogBackup1W<PllRegMacSpec> {
        AnalogBackup1W::new(self, 0)
    }
}
#[doc = "Pll_control_MAC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_mac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_mac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllRegMacSpec;
impl crate::RegisterSpec for PllRegMacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_reg_mac::R`](R) reader structure"]
impl crate::Readable for PllRegMacSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_reg_mac::W`](W) writer structure"]
impl crate::Writable for PllRegMacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets PLL_REG_MAC to value 0"]
impl crate::Resettable for PllRegMacSpec {
    const RESET_VALUE: u32 = 0;
}
