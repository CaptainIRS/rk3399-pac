#[doc = "Register `GRF_IO_VSEL` reader"]
pub type R = crate::R<GrfIoVselSpec>;
#[doc = "Register `GRF_IO_VSEL` writer"]
pub type W = crate::W<GrfIoVselSpec>;
#[doc = "Field `BT656_GPIO2AB_MS` reader - bt656_gpio2ab_ms"]
pub type Bt656Gpio2abMsR = crate::BitReader;
#[doc = "Field `BT656_GPIO2AB_MS` writer - bt656_gpio2ab_ms"]
pub type Bt656Gpio2abMsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDIO_GPIO3D4A_MS` reader - audio_gpio3d4a_ms"]
pub type AudioGpio3d4aMsR = crate::BitReader;
#[doc = "Field `AUDIO_GPIO3D4A_MS` writer - audio_gpio3d4a_ms"]
pub type AudioGpio3d4aMsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC_GPIO4B_MS` reader - sdmmc_gpio4b_ms"]
pub type SdmmcGpio4bMsR = crate::BitReader;
#[doc = "Field `SDMMC_GPIO4B_MS` writer - sdmmc_gpio4b_ms"]
pub type SdmmcGpio4bMsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO1833_GPIO4CD_MS` reader - gpio1833_gpio4cd_ms"]
pub type Gpio1833Gpio4cdMsR = crate::BitReader;
#[doc = "Field `GPIO1833_GPIO4CD_MS` writer - gpio1833_gpio4cd_ms"]
pub type Gpio1833Gpio4cdMsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - bt656_gpio2ab_ms"]
    #[inline(always)]
    pub fn bt656_gpio2ab_ms(&self) -> Bt656Gpio2abMsR {
        Bt656Gpio2abMsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - audio_gpio3d4a_ms"]
    #[inline(always)]
    pub fn audio_gpio3d4a_ms(&self) -> AudioGpio3d4aMsR {
        AudioGpio3d4aMsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sdmmc_gpio4b_ms"]
    #[inline(always)]
    pub fn sdmmc_gpio4b_ms(&self) -> SdmmcGpio4bMsR {
        SdmmcGpio4bMsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gpio1833_gpio4cd_ms"]
    #[inline(always)]
    pub fn gpio1833_gpio4cd_ms(&self) -> Gpio1833Gpio4cdMsR {
        Gpio1833Gpio4cdMsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - bt656_gpio2ab_ms"]
    #[inline(always)]
    #[must_use]
    pub fn bt656_gpio2ab_ms(&mut self) -> Bt656Gpio2abMsW<GrfIoVselSpec> {
        Bt656Gpio2abMsW::new(self, 0)
    }
    #[doc = "Bit 1 - audio_gpio3d4a_ms"]
    #[inline(always)]
    #[must_use]
    pub fn audio_gpio3d4a_ms(&mut self) -> AudioGpio3d4aMsW<GrfIoVselSpec> {
        AudioGpio3d4aMsW::new(self, 1)
    }
    #[doc = "Bit 2 - sdmmc_gpio4b_ms"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_gpio4b_ms(&mut self) -> SdmmcGpio4bMsW<GrfIoVselSpec> {
        SdmmcGpio4bMsW::new(self, 2)
    }
    #[doc = "Bit 3 - gpio1833_gpio4cd_ms"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1833_gpio4cd_ms(&mut self) -> Gpio1833Gpio4cdMsW<GrfIoVselSpec> {
        Gpio1833Gpio4cdMsW::new(self, 3)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfIoVselSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_io_vsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_io_vsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfIoVselSpec;
impl crate::RegisterSpec for GrfIoVselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_io_vsel::R`](R) reader structure"]
impl crate::Readable for GrfIoVselSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_io_vsel::W`](W) writer structure"]
impl crate::Writable for GrfIoVselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_IO_VSEL to value 0"]
impl crate::Resettable for GrfIoVselSpec {
    const RESET_VALUE: u32 = 0;
}
