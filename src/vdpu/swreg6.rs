#[doc = "Register `SWREG6` reader"]
pub type R = crate::R<Swreg6Spec>;
#[doc = "Register `SWREG6` writer"]
pub type W = crate::W<Swreg6Spec>;
#[doc = "Field `SW_SCL_FCT_W_INV` reader - scaling factor of width and ch\n\nvalue =(inputw-1) / (outputw-1)"]
pub type SwSclFctWInvR = crate::FieldReader<u16>;
#[doc = "Field `SW_SCL_FCT_W_INV` writer - scaling factor of width and ch\n\nvalue =(inputw-1) / (outputw-1)"]
pub type SwSclFctWInvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_SCL_FCT_H_INV` reader - scaling factor of height and cv\n\nvalue =(inputw-1) / (outputw-1)"]
pub type SwSclFctHInvR = crate::FieldReader<u16>;
#[doc = "Field `SW_SCL_FCT_H_INV` writer - scaling factor of height and cv\n\nvalue =(inputw-1) / (outputw-1)"]
pub type SwSclFctHInvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - scaling factor of width and ch\n\nvalue =(inputw-1) / (outputw-1)"]
    #[inline(always)]
    pub fn sw_scl_fct_w_inv(&self) -> SwSclFctWInvR {
        SwSclFctWInvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - scaling factor of height and cv\n\nvalue =(inputw-1) / (outputw-1)"]
    #[inline(always)]
    pub fn sw_scl_fct_h_inv(&self) -> SwSclFctHInvR {
        SwSclFctHInvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - scaling factor of width and ch\n\nvalue =(inputw-1) / (outputw-1)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scl_fct_w_inv(&mut self) -> SwSclFctWInvW<Swreg6Spec> {
        SwSclFctWInvW::new(self, 0)
    }
    #[doc = "Bits 16:31 - scaling factor of height and cv\n\nvalue =(inputw-1) / (outputw-1)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scl_fct_h_inv(&mut self) -> SwSclFctHInvW<Swreg6Spec> {
        SwSclFctHInvW::new(self, 16)
    }
}
#[doc = "scl ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg6Spec;
impl crate::RegisterSpec for Swreg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg6::R`](R) reader structure"]
impl crate::Readable for Swreg6Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg6::W`](W) writer structure"]
impl crate::Writable for Swreg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG6 to value 0"]
impl crate::Resettable for Swreg6Spec {
    const RESET_VALUE: u32 = 0;
}
