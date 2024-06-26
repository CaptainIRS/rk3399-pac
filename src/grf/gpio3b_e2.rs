#[doc = "Register `GPIO3B_E2` reader"]
pub type R = crate::R<Gpio3bE2Spec>;
#[doc = "Register `GPIO3B_E2` writer"]
pub type W = crate::W<Gpio3bE2Spec>;
#[doc = "Field `GPIO3B5_E12` reader - GPIO3B5 drive strength control bit1 to bit2"]
pub type Gpio3b5E12R = crate::FieldReader;
#[doc = "Field `GPIO3B5_E12` writer - GPIO3B5 drive strength control bit1 to bit2"]
pub type Gpio3b5E12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO3B6_E` reader - GPIO3B6 drive strength control bit0 to bit2"]
pub type Gpio3b6ER = crate::FieldReader;
#[doc = "Field `GPIO3B6_E` writer - GPIO3B6 drive strength control bit0 to bit2"]
pub type Gpio3b6EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3B7_E` reader - GPIO3B7 drive strength control bit0 to bit2"]
pub type Gpio3b7ER = crate::FieldReader;
#[doc = "Field `GPIO3B7_E` writer - GPIO3B7 drive strength control bit0 to bit2"]
pub type Gpio3b7EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3B5 drive strength control bit1 to bit2"]
    #[inline(always)]
    pub fn gpio3b5_e12(&self) -> Gpio3b5E12R {
        Gpio3b5E12R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - GPIO3B6 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3b6_e(&self) -> Gpio3b6ER {
        Gpio3b6ER::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - GPIO3B7 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3b7_e(&self) -> Gpio3b7ER {
        Gpio3b7ER::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3B5 drive strength control bit1 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b5_e12(&mut self) -> Gpio3b5E12W<Gpio3bE2Spec> {
        Gpio3b5E12W::new(self, 0)
    }
    #[doc = "Bits 2:4 - GPIO3B6 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b6_e(&mut self) -> Gpio3b6EW<Gpio3bE2Spec> {
        Gpio3b6EW::new(self, 2)
    }
    #[doc = "Bits 5:7 - GPIO3B7 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b7_e(&mut self) -> Gpio3b7EW<Gpio3bE2Spec> {
        Gpio3b7EW::new(self, 5)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio3bE2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3b_e2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3b_e2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio3bE2Spec;
impl crate::RegisterSpec for Gpio3bE2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio3b_e2::R`](R) reader structure"]
impl crate::Readable for Gpio3bE2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio3b_e2::W`](W) writer structure"]
impl crate::Writable for Gpio3bE2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO3B_E2 to value 0"]
impl crate::Resettable for Gpio3bE2Spec {
    const RESET_VALUE: u32 = 0;
}
