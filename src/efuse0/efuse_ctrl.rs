#[doc = "Register `EFUSE_CTRL` reader"]
pub type R = crate::R<EfuseCtrlSpec>;
#[doc = "Register `EFUSE_CTRL` writer"]
pub type W = crate::W<EfuseCtrlSpec>;
#[doc = "Field `EFUSE_CSB` reader - efuse chip select enable signal, active low : CSB"]
pub type EfuseCsbR = crate::BitReader;
#[doc = "Field `EFUSE_CSB` writer - efuse chip select enable signal, active low : CSB"]
pub type EfuseCsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_STROBE` reader - efuse turn on the array for read or program access (active high) : STROBE"]
pub type EfuseStrobeR = crate::BitReader;
#[doc = "Field `EFUSE_STROBE` writer - efuse turn on the array for read or program access (active high) : STROBE"]
pub type EfuseStrobeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_LOAD` reader - efuse turn on sense amplifier and load data into latch (active high) : LOAD"]
pub type EfuseLoadR = crate::BitReader;
#[doc = "Field `EFUSE_LOAD` writer - efuse turn on sense amplifier and load data into latch (active high) : LOAD"]
pub type EfuseLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_PGENB` reader - efuse program enable (active low) : PGENB"]
pub type EfusePgenbR = crate::BitReader;
#[doc = "Field `EFUSE_PGENB` writer - efuse program enable (active low) : PGENB"]
pub type EfusePgenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_PS` reader - efuse pass 1.8V program voltage to internal for program(active high) : PS"]
pub type EfusePsR = crate::BitReader;
#[doc = "Field `EFUSE_PS` writer - efuse pass 1.8V program voltage to internal for program(active high) : PS"]
pub type EfusePsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_PD` reader - efuse power down enable (active high) : PD"]
pub type EfusePdR = crate::BitReader;
#[doc = "Field `EFUSE_PD` writer - efuse power down enable (active high) : PD"]
pub type EfusePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_MR` reader - efuse read trip point setting, MR = L for normal read mode; MR = H for margin read1 mode : MR"]
pub type EfuseMrR = crate::BitReader;
#[doc = "Field `EFUSE_MR` writer - efuse read trip point setting, MR = L for normal read mode; MR = H for margin read1 mode : MR"]
pub type EfuseMrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_RSB` reader - efuse redundancy enable(active low) : RSB"]
pub type EfuseRsbR = crate::BitReader;
#[doc = "Field `EFUSE_RSB` writer - efuse redundancy enable(active low) : RSB"]
pub type EfuseRsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_RWL` reader - efuse redundancy information row select (active high) : RWL"]
pub type EfuseRwlR = crate::BitReader;
#[doc = "Field `EFUSE_RWL` writer - efuse redundancy information row select (active high) : RWL"]
pub type EfuseRwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_STROBE_SFT_SEL` reader - efuse strobe control software mode select, active high"]
pub type EfuseStrobeSftSelR = crate::BitReader;
#[doc = "Field `EFUSE_STROBE_SFT_SEL` writer - efuse strobe control software mode select, active high"]
pub type EfuseStrobeSftSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_ADDR` reader - efuse address pins :A\\[9:0\\]"]
pub type EfuseAddrR = crate::FieldReader<u16>;
#[doc = "Field `EFUSE_ADDR` writer - efuse address pins :A\\[9:0\\]"]
pub type EfuseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - efuse chip select enable signal, active low : CSB"]
    #[inline(always)]
    pub fn efuse_csb(&self) -> EfuseCsbR {
        EfuseCsbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - efuse turn on the array for read or program access (active high) : STROBE"]
    #[inline(always)]
    pub fn efuse_strobe(&self) -> EfuseStrobeR {
        EfuseStrobeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - efuse turn on sense amplifier and load data into latch (active high) : LOAD"]
    #[inline(always)]
    pub fn efuse_load(&self) -> EfuseLoadR {
        EfuseLoadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - efuse program enable (active low) : PGENB"]
    #[inline(always)]
    pub fn efuse_pgenb(&self) -> EfusePgenbR {
        EfusePgenbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - efuse pass 1.8V program voltage to internal for program(active high) : PS"]
    #[inline(always)]
    pub fn efuse_ps(&self) -> EfusePsR {
        EfusePsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - efuse power down enable (active high) : PD"]
    #[inline(always)]
    pub fn efuse_pd(&self) -> EfusePdR {
        EfusePdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - efuse read trip point setting, MR = L for normal read mode; MR = H for margin read1 mode : MR"]
    #[inline(always)]
    pub fn efuse_mr(&self) -> EfuseMrR {
        EfuseMrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - efuse redundancy enable(active low) : RSB"]
    #[inline(always)]
    pub fn efuse_rsb(&self) -> EfuseRsbR {
        EfuseRsbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - efuse redundancy information row select (active high) : RWL"]
    #[inline(always)]
    pub fn efuse_rwl(&self) -> EfuseRwlR {
        EfuseRwlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - efuse strobe control software mode select, active high"]
    #[inline(always)]
    pub fn efuse_strobe_sft_sel(&self) -> EfuseStrobeSftSelR {
        EfuseStrobeSftSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:25 - efuse address pins :A\\[9:0\\]"]
    #[inline(always)]
    pub fn efuse_addr(&self) -> EfuseAddrR {
        EfuseAddrR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - efuse chip select enable signal, active low : CSB"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_csb(&mut self) -> EfuseCsbW<EfuseCtrlSpec> {
        EfuseCsbW::new(self, 0)
    }
    #[doc = "Bit 1 - efuse turn on the array for read or program access (active high) : STROBE"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_strobe(&mut self) -> EfuseStrobeW<EfuseCtrlSpec> {
        EfuseStrobeW::new(self, 1)
    }
    #[doc = "Bit 2 - efuse turn on sense amplifier and load data into latch (active high) : LOAD"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_load(&mut self) -> EfuseLoadW<EfuseCtrlSpec> {
        EfuseLoadW::new(self, 2)
    }
    #[doc = "Bit 3 - efuse program enable (active low) : PGENB"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_pgenb(&mut self) -> EfusePgenbW<EfuseCtrlSpec> {
        EfusePgenbW::new(self, 3)
    }
    #[doc = "Bit 4 - efuse pass 1.8V program voltage to internal for program(active high) : PS"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_ps(&mut self) -> EfusePsW<EfuseCtrlSpec> {
        EfusePsW::new(self, 4)
    }
    #[doc = "Bit 5 - efuse power down enable (active high) : PD"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_pd(&mut self) -> EfusePdW<EfuseCtrlSpec> {
        EfusePdW::new(self, 5)
    }
    #[doc = "Bit 6 - efuse read trip point setting, MR = L for normal read mode; MR = H for margin read1 mode : MR"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_mr(&mut self) -> EfuseMrW<EfuseCtrlSpec> {
        EfuseMrW::new(self, 6)
    }
    #[doc = "Bit 7 - efuse redundancy enable(active low) : RSB"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_rsb(&mut self) -> EfuseRsbW<EfuseCtrlSpec> {
        EfuseRsbW::new(self, 7)
    }
    #[doc = "Bit 8 - efuse redundancy information row select (active high) : RWL"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_rwl(&mut self) -> EfuseRwlW<EfuseCtrlSpec> {
        EfuseRwlW::new(self, 8)
    }
    #[doc = "Bit 9 - efuse strobe control software mode select, active high"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_strobe_sft_sel(&mut self) -> EfuseStrobeSftSelW<EfuseCtrlSpec> {
        EfuseStrobeSftSelW::new(self, 9)
    }
    #[doc = "Bits 16:25 - efuse address pins :A\\[9:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_addr(&mut self) -> EfuseAddrW<EfuseCtrlSpec> {
        EfuseAddrW::new(self, 16)
    }
}
#[doc = "e fuse control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseCtrlSpec;
impl crate::RegisterSpec for EfuseCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_ctrl::R`](R) reader structure"]
impl crate::Readable for EfuseCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse_ctrl::W`](W) writer structure"]
impl crate::Writable for EfuseCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_CTRL to value 0"]
impl crate::Resettable for EfuseCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
