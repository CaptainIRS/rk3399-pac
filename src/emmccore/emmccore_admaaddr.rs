#[doc = "Register `EMMCCORE_ADMAADDR` reader"]
pub type R = crate::R<EmmccoreAdmaaddrSpec>;
#[doc = "Register `EMMCCORE_ADMAADDR` writer"]
pub type W = crate::W<EmmccoreAdmaaddrSpec>;
#[doc = "Field `ADDRL32` reader - ADMA System Address \\[31:0\\]. This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32-bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 0."]
pub type Addrl32R = crate::FieldReader<u32>;
#[doc = "Field `ADDRL32` writer - ADMA System Address \\[31:0\\]. This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32-bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 0."]
pub type Addrl32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[doc = "Field `ADDRH32` reader - ADMA System Address \\[63:32\\]."]
pub type Addrh32R = crate::FieldReader<u32>;
#[doc = "Field `ADDRH32` writer - ADMA System Address \\[63:32\\]."]
pub type Addrh32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address \\[31:0\\]. This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32-bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 0."]
    #[inline(always)]
    pub fn addrl32(&self) -> Addrl32R {
        Addrl32R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bits 32:63 - ADMA System Address \\[63:32\\]."]
    #[inline(always)]
    pub fn addrh32(&self) -> Addrh32R {
        Addrh32R::new(((self.bits >> 32) & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address \\[31:0\\]. This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32-bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 0."]
    #[inline(always)]
    #[must_use]
    pub fn addrl32(&mut self) -> Addrl32W<EmmccoreAdmaaddrSpec> {
        Addrl32W::new(self, 0)
    }
    #[doc = "Bits 32:63 - ADMA System Address \\[63:32\\]."]
    #[inline(always)]
    #[must_use]
    pub fn addrh32(&mut self) -> Addrh32W<EmmccoreAdmaaddrSpec> {
        Addrh32W::new(self, 32)
    }
}
#[doc = "ADMA system address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_admaaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_admaaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreAdmaaddrSpec;
impl crate::RegisterSpec for EmmccoreAdmaaddrSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`emmccore_admaaddr::R`](R) reader structure"]
impl crate::Readable for EmmccoreAdmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_admaaddr::W`](W) writer structure"]
impl crate::Writable for EmmccoreAdmaaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_ADMAADDR to value 0"]
impl crate::Resettable for EmmccoreAdmaaddrSpec {
    const RESET_VALUE: u64 = 0;
}
