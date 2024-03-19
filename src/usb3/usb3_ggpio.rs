#[doc = "Register `USB3_GGPIO` reader"]
pub type R = crate::R<Usb3GgpioSpec>;
#[doc = "Register `USB3_GGPIO` writer"]
pub type W = crate::W<Usb3GgpioSpec>;
#[doc = "Field `GPI` reader - General Purpose Input\n\nThis field's read value reflects the gp_in\\[15:0\\]
core input value."]
pub type GpiR = crate::FieldReader<u16>;
#[doc = "Field `GPO` reader - General Purpose Output\n\nThis field's value is driven out on the gp_out\\[15:0\\]
core output\n\nport."]
pub type GpoR = crate::FieldReader<u16>;
#[doc = "Field `GPO` writer - General Purpose Output\n\nThis field's value is driven out on the gp_out\\[15:0\\]
core output\n\nport."]
pub type GpoW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - General Purpose Input\n\nThis field's read value reflects the gp_in\\[15:0\\]
core input value."]
    #[inline(always)]
    pub fn gpi(&self) -> GpiR {
        GpiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - General Purpose Output\n\nThis field's value is driven out on the gp_out\\[15:0\\]
core output\n\nport."]
    #[inline(always)]
    pub fn gpo(&self) -> GpoR {
        GpoR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - General Purpose Output\n\nThis field's value is driven out on the gp_out\\[15:0\\]
core output\n\nport."]
    #[inline(always)]
    #[must_use]
    pub fn gpo(&mut self) -> GpoW<Usb3GgpioSpec> {
        GpoW::new(self, 16)
    }
}
#[doc = "Global General Purpose Input/Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_ggpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_ggpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GgpioSpec;
impl crate::RegisterSpec for Usb3GgpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_ggpio::R`](R) reader structure"]
impl crate::Readable for Usb3GgpioSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_ggpio::W`](W) writer structure"]
impl crate::Writable for Usb3GgpioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GGPIO to value 0"]
impl crate::Resettable for Usb3GgpioSpec {
    const RESET_VALUE: u32 = 0;
}
