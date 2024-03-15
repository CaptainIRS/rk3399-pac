#[doc = "Register `IH_AHBDMAAUD_STAT0` reader"]
pub type R = crate::R<IhAhbdmaaudStat0Spec>;
#[doc = "Register `IH_AHBDMAAUD_STAT0` writer"]
pub type W = crate::W<IhAhbdmaaudStat0Spec>;
#[doc = "Field `AHBDMAAUD_INTLOSTOWNERSHIP` reader - AHB audio DMA lost ownership interrupt"]
pub type AhbdmaaudIntlostownershipR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTLOSTOWNERSHIP` writer - AHB audio DMA lost ownership interrupt"]
pub type AhbdmaaudIntlostownershipW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTERROR` reader - AHB audio DMA error interrupt"]
pub type AhbdmaaudInterrorR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTERROR` writer - AHB audio DMA error interrupt"]
pub type AhbdmaaudInterrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTBUFFOVERRUN` reader - AHB audio DMA Buffer overrun interruption"]
pub type AhbdmaaudIntbuffoverrunR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTBUFFOVERRUN` writer - AHB audio DMA Buffer overrun interruption"]
pub type AhbdmaaudIntbuffoverrunW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 4 - AHB audio DMA lost ownership interrupt"]
    #[inline(always)]
    pub fn ahbdmaaud_intlostownership(&self) -> AhbdmaaudIntlostownershipR {
        AhbdmaaudIntlostownershipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB audio DMA error interrupt"]
    #[inline(always)]
    pub fn ahbdmaaud_interror(&self) -> AhbdmaaudInterrorR {
        AhbdmaaudInterrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHB audio DMA Buffer overrun interruption"]
    #[inline(always)]
    pub fn ahbdmaaud_intbuffoverrun(&self) -> AhbdmaaudIntbuffoverrunR {
        AhbdmaaudIntbuffoverrunR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - AHB audio DMA lost ownership interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intlostownership(
        &mut self,
    ) -> AhbdmaaudIntlostownershipW<IhAhbdmaaudStat0Spec> {
        AhbdmaaudIntlostownershipW::new(self, 4)
    }
    #[doc = "Bit 5 - AHB audio DMA error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_interror(&mut self) -> AhbdmaaudInterrorW<IhAhbdmaaudStat0Spec> {
        AhbdmaaudInterrorW::new(self, 5)
    }
    #[doc = "Bit 6 - AHB audio DMA Buffer overrun interruption"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intbuffoverrun(&mut self) -> AhbdmaaudIntbuffoverrunW<IhAhbdmaaudStat0Spec> {
        AhbdmaaudIntbuffoverrunW::new(self, 6)
    }
}
#[doc = "AHB audio DMA lost ownership interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_ahbdmaaud_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_ahbdmaaud_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhAhbdmaaudStat0Spec;
impl crate::RegisterSpec for IhAhbdmaaudStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_ahbdmaaud_stat0::R`](R) reader structure"]
impl crate::Readable for IhAhbdmaaudStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_ahbdmaaud_stat0::W`](W) writer structure"]
impl crate::Writable for IhAhbdmaaudStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x70;
}
#[doc = "`reset()` method sets IH_AHBDMAAUD_STAT0 to value 0"]
impl crate::Resettable for IhAhbdmaaudStat0Spec {
    const RESET_VALUE: u8 = 0;
}
