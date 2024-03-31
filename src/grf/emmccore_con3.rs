#[doc = "Register `EMMCCORE_CON3` reader"]
pub type R = crate::R<EmmccoreCon3Spec>;
#[doc = "Register `EMMCCORE_CON3` writer"]
pub type W = crate::W<EmmccoreCon3Spec>;
#[doc = "Field `EMMCCORE_CON3` reader - emmc controller control register 3"]
pub type EmmccoreCon3R = crate::FieldReader<u16>;
#[doc = "Field `EMMCCORE_CON3` writer - emmc controller control register 3"]
pub type EmmccoreCon3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - emmc controller control register 3"]
    #[inline(always)]
    pub fn emmccore_con3(&self) -> EmmccoreCon3R {
        EmmccoreCon3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - emmc controller control register 3"]
    #[inline(always)]
    #[must_use]
    pub fn emmccore_con3(&mut self) -> EmmccoreCon3W<EmmccoreCon3Spec> {
        EmmccoreCon3W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<EmmccoreCon3Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCon3Spec;
impl crate::RegisterSpec for EmmccoreCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_con3::R`](R) reader structure"]
impl crate::Readable for EmmccoreCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_con3::W`](W) writer structure"]
impl crate::Writable for EmmccoreCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CON3 to value 0"]
impl crate::Resettable for EmmccoreCon3Spec {
    const RESET_VALUE: u32 = 0;
}
