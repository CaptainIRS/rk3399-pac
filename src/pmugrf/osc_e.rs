#[doc = "Register `OSC_E` reader"]
pub type R = crate::R<OscESpec>;
#[doc = "Register `OSC_E` writer"]
pub type W = crate::W<OscESpec>;
#[doc = "Field `OSC_E` reader - 24M OSC drive strenth"]
pub type OscER = crate::FieldReader;
#[doc = "Field `OSC_E` writer - 24M OSC drive strenth"]
pub type OscEW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_ENABLE` reader - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\nWhen bit 18=1, bit 2 can be written by\n\nsoftware .\n\nWhen bit 18=0, bit 2 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader;
#[doc = "Field `WRITE_ENABLE` writer - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\nWhen bit 18=1, bit 2 can be written by\n\nsoftware .\n\nWhen bit 18=0, bit 2 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 24M OSC drive strenth"]
    #[inline(always)]
    pub fn osc_e(&self) -> OscER {
        OscER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\nWhen bit 18=1, bit 2 can be written by\n\nsoftware .\n\nWhen bit 18=0, bit 2 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 24M OSC drive strenth"]
    #[inline(always)]
    #[must_use]
    pub fn osc_e(&mut self) -> OscEW<OscESpec> {
        OscEW::new(self, 0)
    }
    #[doc = "Bits 16:18 - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\nWhen bit 18=1, bit 2 can be written by\n\nsoftware .\n\nWhen bit 18=0, bit 2 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<OscESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "OSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscESpec;
impl crate::RegisterSpec for OscESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc_e::R`](R) reader structure"]
impl crate::Readable for OscESpec {}
#[doc = "`write(|w| ..)` method takes [`osc_e::W`](W) writer structure"]
impl crate::Writable for OscESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSC_E to value 0x06"]
impl crate::Resettable for OscESpec {
    const RESET_VALUE: u32 = 0x06;
}
