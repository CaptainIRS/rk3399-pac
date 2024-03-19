#[doc = "Register `DDR_PI_REG_71` reader"]
pub type R = crate::R<DdrPiReg71Spec>;
#[doc = "Register `DDR_PI_REG_71` writer"]
pub type W = crate::W<DdrPiReg71Spec>;
#[doc = "Field `PI_ODTLON_F0` reader - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f0\" of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiOdtlonF0R = crate::FieldReader;
#[doc = "Field `PI_ODTLON_F0` writer - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f0\" of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiOdtlonF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTON_MIN_F0` reader - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTodtonMinF0R = crate::FieldReader;
#[doc = "Field `PI_TODTON_MIN_F0` writer - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTodtonMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_ODTLON_F1` reader - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f1\" of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiOdtlonF1R = crate::FieldReader;
#[doc = "Field `PI_ODTLON_F1` writer - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f1\" of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiOdtlonF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTON_MIN_F1` reader - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTodtonMinF1R = crate::FieldReader;
#[doc = "Field `PI_TODTON_MIN_F1` writer - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTodtonMinF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f0\" of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    pub fn pi_odtlon_f0(&self) -> PiOdtlonF0R {
        PiOdtlonF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_todton_min_f0(&self) -> PiTodtonMinF0R {
        PiTodtonMinF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f1\" of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    pub fn pi_odtlon_f1(&self) -> PiOdtlonF1R {
        PiOdtlonF1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_todton_min_f1(&self) -> PiTodtonMinF1R {
        PiTodtonMinF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f0\" of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odtlon_f0(&mut self) -> PiOdtlonF0W<DdrPiReg71Spec> {
        PiOdtlonF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todton_min_f0(&mut self) -> PiTodtonMinF0W<DdrPiReg71Spec> {
        PiTodtonMinF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the latency from a CAS-2 command to the tODTon\n\nreference. The suffix \"_f1\" of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odtlon_f1(&mut self) -> PiOdtlonF1W<DdrPiReg71Spec> {
        PiOdtlonF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the point in time when the device termination circuit leaves\n\nHigh-Z and ODT resistance begins to turn on. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todton_min_f1(&mut self) -> PiTodtonMinF1W<DdrPiReg71Spec> {
        PiTodtonMinF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_71::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_71::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg71Spec;
impl crate::RegisterSpec for DdrPiReg71Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_71::R`](R) reader structure"]
impl crate::Readable for DdrPiReg71Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_71::W`](W) writer structure"]
impl crate::Writable for DdrPiReg71Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_71 to value 0"]
impl crate::Resettable for DdrPiReg71Spec {
    const RESET_VALUE: u32 = 0;
}
