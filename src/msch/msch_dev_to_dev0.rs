#[doc = "Register `MSCH_DevToDev0` reader"]
pub type R = crate::R<MschDevToDev0Spec>;
#[doc = "Register `MSCH_DevToDev0` writer"]
pub type W = crate::W<MschDevToDev0Spec>;
#[doc = "Field `BUSRDTORD` reader - number of cycle between the last read data of a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BusrdtordR = crate::FieldReader;
#[doc = "Field `BUSRDTORD` writer - number of cycle between the last read data of a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BusrdtordW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUSRDTOWR` reader - number of cycle between the last read data of a device and the first\n\nwrite data to another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BusrdtowrR = crate::FieldReader;
#[doc = "Field `BUSRDTOWR` writer - number of cycle between the last read data of a device and the first\n\nwrite data to another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BusrdtowrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUSWRTORD` reader - number of cycle between the last write data to a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BuswrtordR = crate::FieldReader;
#[doc = "Field `BUSWRTORD` writer - number of cycle between the last write data to a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BuswrtordW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUSWRTOWR` reader - number of cycle between the last write data to a device and the first\n\nwrite data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BuswrtowrR = crate::FieldReader;
#[doc = "Field `BUSWRTOWR` writer - number of cycle between the last write data to a device and the first\n\nwrite data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
pub type BuswrtowrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - number of cycle between the last read data of a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    pub fn busrdtord(&self) -> BusrdtordR {
        BusrdtordR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - number of cycle between the last read data of a device and the first\n\nwrite data to another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    pub fn busrdtowr(&self) -> BusrdtowrR {
        BusrdtowrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - number of cycle between the last write data to a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    pub fn buswrtord(&self) -> BuswrtordR {
        BuswrtordR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - number of cycle between the last write data to a device and the first\n\nwrite data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    pub fn buswrtowr(&self) -> BuswrtowrR {
        BuswrtowrR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - number of cycle between the last read data of a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    #[must_use]
    pub fn busrdtord(&mut self) -> BusrdtordW<MschDevToDev0Spec> {
        BusrdtordW::new(self, 0)
    }
    #[doc = "Bits 4:6 - number of cycle between the last read data of a device and the first\n\nwrite data to another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    #[must_use]
    pub fn busrdtowr(&mut self) -> BusrdtowrW<MschDevToDev0Spec> {
        BusrdtowrW::new(self, 4)
    }
    #[doc = "Bits 8:10 - number of cycle between the last write data to a device and the first\n\nread data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    #[must_use]
    pub fn buswrtord(&mut self) -> BuswrtordW<MschDevToDev0Spec> {
        BuswrtordW::new(self, 8)
    }
    #[doc = "Bits 12:14 - number of cycle between the last write data to a device and the first\n\nwrite data of another device.\n\nThe field must be set according to the third-party DDR controller\n\nspecification."]
    #[inline(always)]
    #[must_use]
    pub fn buswrtowr(&mut self) -> BuswrtowrW<MschDevToDev0Spec> {
        BuswrtowrW::new(self, 12)
    }
}
#[doc = "Timing values concerning device to device data bus ownership c\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_dev_to_dev0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_dev_to_dev0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschDevToDev0Spec;
impl crate::RegisterSpec for MschDevToDev0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_dev_to_dev0::R`](R) reader structure"]
impl crate::Readable for MschDevToDev0Spec {}
#[doc = "`write(|w| ..)` method takes [`msch_dev_to_dev0::W`](W) writer structure"]
impl crate::Writable for MschDevToDev0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCH_DevToDev0 to value 0x0222"]
impl crate::Resettable for MschDevToDev0Spec {
    const RESET_VALUE: u32 = 0x0222;
}
