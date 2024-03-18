#[doc = "Register `PI_REG_23` reader"]
pub type R = crate::R<PiReg23Spec>;
#[doc = "Register `PI_REG_23` writer"]
pub type W = crate::W<PiReg23Spec>;
#[doc = "Field `PI_FREQ_MAP` reader - User indicates all the supported working frequencies. Each bit represents one supported frequency."]
pub type PiFreqMapR = crate::FieldReader<u32>;
#[doc = "Field `PI_FREQ_MAP` writer - User indicates all the supported working frequencies. Each bit represents one supported frequency."]
pub type PiFreqMapW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User indicates all the supported working frequencies. Each bit represents one supported frequency."]
    #[inline(always)]
    pub fn pi_freq_map(&self) -> PiFreqMapR {
        PiFreqMapR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User indicates all the supported working frequencies. Each bit represents one supported frequency."]
    #[inline(always)]
    #[must_use]
    pub fn pi_freq_map(&mut self) -> PiFreqMapW<PiReg23Spec> {
        PiFreqMapW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg23Spec;
impl crate::RegisterSpec for PiReg23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_23::R`](R) reader structure"]
impl crate::Readable for PiReg23Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_23::W`](W) writer structure"]
impl crate::Writable for PiReg23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_23 to value 0"]
impl crate::Resettable for PiReg23Spec {
    const RESET_VALUE: u32 = 0;
}
