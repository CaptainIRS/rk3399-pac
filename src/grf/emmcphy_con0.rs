#[doc = "Register `EMMCPHY_CON0` reader"]
pub type R = crate::R<EmmcphyCon0Spec>;
#[doc = "Register `EMMCPHY_CON0` writer"]
pub type W = crate::W<EmmcphyCon0Spec>;
#[doc = "Field `EMMCPHY_CON0` reader - emmc phy control register 0"]
pub type EmmcphyCon0R = crate::FieldReader<u16>;
#[doc = "Field `EMMCPHY_CON0` writer - emmc phy control register 0"]
pub type EmmcphyCon0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - emmc phy control register 0"]
    #[inline(always)]
    pub fn emmcphy_con0(&self) -> EmmcphyCon0R {
        EmmcphyCon0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - emmc phy control register 0"]
    #[inline(always)]
    #[must_use]
    pub fn emmcphy_con0(&mut self) -> EmmcphyCon0W<EmmcphyCon0Spec> {
        EmmcphyCon0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<EmmcphyCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmcphyCon0Spec;
impl crate::RegisterSpec for EmmcphyCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmcphy_con0::R`](R) reader structure"]
impl crate::Readable for EmmcphyCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`emmcphy_con0::W`](W) writer structure"]
impl crate::Writable for EmmcphyCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCPHY_CON0 to value 0"]
impl crate::Resettable for EmmcphyCon0Spec {
    const RESET_VALUE: u32 = 0;
}
