#[doc = "Register `GRF_GPIO3C_E01` reader"]
pub type R = crate::R<GrfGpio3cE01Spec>;
#[doc = "Register `GRF_GPIO3C_E01` writer"]
pub type W = crate::W<GrfGpio3cE01Spec>;
#[doc = "Field `GPIO3C0_E` reader - GPIO3C0 drive strength control bit0 to bit2"]
pub type Gpio3c0ER = crate::FieldReader;
#[doc = "Field `GPIO3C0_E` writer - GPIO3C0 drive strength control bit0 to bit2"]
pub type Gpio3c0EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3C1_E` reader - GPIO3C1 drive strength control bit0 to bit2"]
pub type Gpio3c1ER = crate::FieldReader;
#[doc = "Field `GPIO3C1_E` writer - GPIO3C1 drive strength control bit0 to bit2"]
pub type Gpio3c1EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3C2_E` reader - GPIO3C2 drive strength control bit0 to bit2"]
pub type Gpio3c2ER = crate::FieldReader;
#[doc = "Field `GPIO3C2_E` writer - GPIO3C2 drive strength control bit0 to bit2"]
pub type Gpio3c2EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3C3_E` reader - GPIO3C3 drive strength control bit0 to bit2"]
pub type Gpio3c3ER = crate::FieldReader;
#[doc = "Field `GPIO3C3_E` writer - GPIO3C3 drive strength control bit0 to bit2"]
pub type Gpio3c3EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3C4_E` reader - GPIO3C4 drive strength control bit0 to bit2"]
pub type Gpio3c4ER = crate::FieldReader;
#[doc = "Field `GPIO3C4_E` writer - GPIO3C4 drive strength control bit0 to bit2"]
pub type Gpio3c4EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3C5_E0` reader - GPIO3C5 drive strength control bit0"]
pub type Gpio3c5E0R = crate::BitReader;
#[doc = "Field `GPIO3C5_E0` writer - GPIO3C5 drive strength control bit0"]
pub type Gpio3c5E0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - GPIO3C0 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3c0_e(&self) -> Gpio3c0ER {
        Gpio3c0ER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - GPIO3C1 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3c1_e(&self) -> Gpio3c1ER {
        Gpio3c1ER::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - GPIO3C2 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3c2_e(&self) -> Gpio3c2ER {
        Gpio3c2ER::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - GPIO3C3 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3c3_e(&self) -> Gpio3c3ER {
        Gpio3c3ER::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - GPIO3C4 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3c4_e(&self) -> Gpio3c4ER {
        Gpio3c4ER::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - GPIO3C5 drive strength control bit0"]
    #[inline(always)]
    pub fn gpio3c5_e0(&self) -> Gpio3c5E0R {
        Gpio3c5E0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO3C0 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c0_e(&mut self) -> Gpio3c0EW<GrfGpio3cE01Spec> {
        Gpio3c0EW::new(self, 0)
    }
    #[doc = "Bits 3:5 - GPIO3C1 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c1_e(&mut self) -> Gpio3c1EW<GrfGpio3cE01Spec> {
        Gpio3c1EW::new(self, 3)
    }
    #[doc = "Bits 6:8 - GPIO3C2 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c2_e(&mut self) -> Gpio3c2EW<GrfGpio3cE01Spec> {
        Gpio3c2EW::new(self, 6)
    }
    #[doc = "Bits 9:11 - GPIO3C3 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c3_e(&mut self) -> Gpio3c3EW<GrfGpio3cE01Spec> {
        Gpio3c3EW::new(self, 9)
    }
    #[doc = "Bits 12:14 - GPIO3C4 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c4_e(&mut self) -> Gpio3c4EW<GrfGpio3cE01Spec> {
        Gpio3c4EW::new(self, 12)
    }
    #[doc = "Bit 15 - GPIO3C5 drive strength control bit0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c5_e0(&mut self) -> Gpio3c5E0W<GrfGpio3cE01Spec> {
        Gpio3c5E0W::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3cE01Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_e01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_e01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3cE01Spec;
impl crate::RegisterSpec for GrfGpio3cE01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3c_e01::R`](R) reader structure"]
impl crate::Readable for GrfGpio3cE01Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3c_e01::W`](W) writer structure"]
impl crate::Writable for GrfGpio3cE01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3C_E01 to value 0"]
impl crate::Resettable for GrfGpio3cE01Spec {
    const RESET_VALUE: u32 = 0;
}
